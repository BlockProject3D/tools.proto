// Copyright (c) 2024, BlockProject 3D
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright notice,
//       this list of conditions and the following disclaimer in the documentation
//       and/or other materials provided with the distribution.
//     * Neither the name of BlockProject 3D nor the names of its contributors
//       may be used to endorse or promote products derived from this software
//       without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::rc::Rc;
use crate::compiler::message::{Referenced, SizeInfo};
use crate::compiler::{Error, Protocol};
use crate::compiler::structure::{Field, FieldView, FixedField, Structure};

#[derive(Clone, Debug)]
pub struct UnionField {
    pub name: String,
    pub case: usize,
    pub item_type: Option<Referenced>
}

impl UnionField {
    pub fn from_model(proto: &Protocol, discriminant: &FixedField, value: crate::model::union::UnionField) -> Result<Self, Error> {
        let case: usize = match &discriminant.view {
            FieldView::Float { .. } => return Err(Error::FloatInUnionDiscriminant),
            FieldView::Enum(v) => v.variants_map.get(&value.case).map(|v| *v).ok_or_else(|| Error::InvalidUnionCase(value.case))?,
            FieldView::Transmute | FieldView::SignedCast { .. } => {
                let value: isize = value.case.parse().map_err(|_| Error::InvalidUnionCase(value.case))?;
                value as usize
            }
            FieldView::None => value.case.parse().map_err(|_| Error::InvalidUnionCase(value.case))?
        };
        let item_type = value.item_type.map(|v| Referenced::lookup(proto, &v).ok_or_else(|| Error::UndefinedReference(v)))
            .transpose()?;
        Ok(UnionField {
            name: value.name,
            case,
            item_type
        })
    }
}

#[derive(Clone, Debug)]
pub struct DiscriminantField {
    pub root: Rc<Structure>,
    pub leaf: Rc<Structure>,
    pub leaf_index: usize,
    pub index_list: Vec<usize>
}

struct DiscriminantFieldIterator<'a> {
    cur: &'a Structure,
    index: std::slice::Iter<'a, usize>
}

impl<'a> Iterator for DiscriminantFieldIterator<'a> {
    type Item = (&'a Field, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index.next()?;
        let field = &self.cur.fields[*index];
        let is_leaf = match field {
            Field::Fixed(_) => true,
            Field::Array(_) => std::unreachable!(),
            Field::Struct(v) => {
                self.cur = &v.r;
                false
            }
        };
        Some((field, is_leaf))
    }
}

impl DiscriminantField {
    pub fn iter(&self) -> impl Iterator<Item = (&Field, bool)> {
        DiscriminantFieldIterator {
            cur: &self.root,
            index: self.index_list.iter()
        }
    }

    pub fn get_leaf(&self) -> &FixedField {
        self.leaf.fields[self.leaf_index].as_fixed().unwrap()
    }

    pub fn from_model(proto: &Protocol, discriminant: String) -> Result<Self, Error> {
        let mut parts = discriminant.split(".");
        let name = parts.next().ok_or(Error::InvalidUnionDiscriminant)?;
        let mut leaf = proto.structs_by_name.get(name).ok_or_else(|| Error::UndefinedReference(name.into()))?;
        let root = leaf;
        let mut index_list = Vec::new();
        for sub in parts {
            let (index, field) = leaf.fields.iter().enumerate()
                .find(|(_, v)| v.name() == sub)
                .ok_or_else(|| Error::UndefinedReference(format!("{}.{}", name, sub)))?;
            index_list.push(index);
            match field {
                Field::Fixed(_) => break,
                Field::Struct(v) => leaf = &v.r,
                Field::Array(_) => return Err(Error::InvalidUnionDiscriminant)
            }
        }
        Ok(DiscriminantField {
            root: root.clone(),
            leaf: leaf.clone(),
            leaf_index: index_list.last().map(|v| *v).unwrap(),
            index_list
        })
    }
}

#[derive(Clone, Debug)]
pub struct Union  {
    pub name: String,
    pub discriminant: DiscriminantField,
    pub cases: Vec<UnionField>,
    pub size: SizeInfo
}

impl Union {
    pub fn from_model(proto: &Protocol, value: crate::model::union::Union) -> Result<Self, Error> {
        let discriminant = DiscriminantField::from_model(proto, value.discriminant)?;
        let cases = value.cases.into_iter().map(|v| UnionField::from_model(proto, discriminant.get_leaf(), v))
            .collect::<Result<Vec<UnionField>, Error>>()?;
        let is_element_dyn_sized = cases.iter().any(|v| v.item_type.as_ref().map(|v| match v {
            Referenced::Struct(_) => false,
            Referenced::Message(v) => v.size.is_element_dyn_sized
        }).unwrap_or_default());
        let is_dyn_sized = cases.iter().any(|v| v.item_type.as_ref().map(|v| match v {
            Referenced::Struct(_) => false,
            Referenced::Message(v) => v.size.is_dyn_sized
        }).unwrap_or_default());
        Ok(Union {
            name: value.name,
            discriminant,
            cases,
            size: SizeInfo {
                is_element_dyn_sized,
                is_dyn_sized
            }
        })
    }
}
