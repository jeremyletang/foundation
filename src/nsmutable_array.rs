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

use nsarray::Array;

use std::fmt::{Show, Formatter, FormatError};
use objcruntime;

pub struct NSMutableArray<T> {
    id: objcruntime::id
}

impl<T> NSMutableArray<T> {
    pub fn new() -> NSMutableArray<T> {
        NSMutableArray {
            id: m![m![cls!(NSMutableArray) alloc] init]
        }
    }

    pub fn add(&mut self, item: T) {
        m![self.id addObject: item];
    }
}

impl<T: objcruntime::Id> Array<T> for NSMutableArray<T> {}

impl<T: Show> Show for NSMutableArray<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        write!(f, "{}", "unimplemented")
    }
}

impl<T> Collection for NSMutableArray<T> {
    fn len(&self) -> uint {
        unsafe { ::std::mem::transmute(m![self.id count]) }
    }
}

impl<T> objcruntime::Id for NSMutableArray<T> {
    fn as_id(&self) -> objcruntime::id {
        self.id
    }

    fn from_id(id: objcruntime::id) -> NSMutableArray<T> {
        NSMutableArray {
            id: id
        }
    }
}

impl<T> Mutable for NSMutableArray<T> {
    fn clear(&mut self) {
        m![self.id removeAllObjects];
    }
}

#[unsafe_destructor]
impl<T> Drop for NSMutableArray<T> {
    fn drop(&mut self) {
        m![self.id release];
    }
}
