// The MIT License (MIT)
//
// Copyright (c) 2014 Jeremy Letang
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::fmt::{Show, Formatter, FormatError};
use objcruntime;

pub struct NSString {
    id: objcruntime::id
}

impl NSString {
    pub fn new() -> NSString {
        NSString {
            id: m![m![cls!(NSString) alloc] init]
        }
    }

    pub fn from_str(string: &str) -> NSString {
        NSString {
            id: m![m![cls!(NSString) alloc] initWithUTF8String: cstr!(string)]
        }
    }

    pub fn from_string(string: String) -> NSString {
        NSString {
            id: m![m![cls!(NSString) alloc] initWithUTF8String: cstr!(string.as_slice())]
        }
    }

    pub fn from_nsstring(string: &NSString) -> NSString {
        NSString {
            id: m![m![cls!(NSString) alloc] initWithUTF8String: cstr!(string.utf8())]
        }
    }

    pub fn utf8(&self) -> &'static str {
        let utf8 = m![self.id UTF8String];
        unsafe { ::std::str::raw::c_str_to_static_slice(::std::mem::transmute(utf8)) }
    }

    pub fn to_string(&self) -> String {
        String::from_str(self.utf8())
    }
}

impl Show for NSString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        write!(f, "{}", self.utf8())
    }
}

impl Collection for NSString {
    fn len(&self) -> uint {
        unsafe { ::std::mem::transmute(m![self.id length]) }
    }
}

impl objcruntime::Id for NSString {
    fn as_id(&self) -> objcruntime::id {
        self.id
    }

    fn from_id(id: objcruntime::id) -> NSString {
        NSString {
            id: id
        }
    }
}

impl Drop for NSString {
    fn drop(&mut self) {
        m![self.id release];
    }
}
