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

use std::collections::HashMap;
use std::rc::Rc;
use crate::compiler::error::CompilerError;
use crate::compiler::message::Message;
use crate::compiler::structure::Structure;

#[derive(Clone, Debug)]
pub struct Protocol {
    pub structs_by_name: HashMap<String, Rc<Structure>>,
    pub messages_by_name: HashMap<String, Rc<Message>>,
    pub structs: Vec<Rc<Structure>>,
    pub messages: Vec<Rc<Message>>

}

impl Protocol {
    pub fn from_model(value: crate::model::Protocol) -> Result<Self, CompilerError> {
        let mut proto = Protocol {
            structs_by_name: HashMap::new(),
            messages_by_name: HashMap::new(),
            structs: Vec::new(),
            messages: Vec::new()
        };
        for v in value.structs {
            let v = Rc::new(Structure::from_model(&proto, v)?);
            proto.structs_by_name.insert(v.name.clone(), v.clone());
            proto.structs.push(v);
        }
        for v in value.messages {
            let v = Rc::new(Message::from_model(&proto, v)?);
            proto.messages_by_name.insert(v.name.clone(), v.clone());
            proto.messages.push(v);
        }
        Ok(proto)
    }
}
