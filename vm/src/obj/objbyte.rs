use crate::obj::objtype::PyClassRef;
use crate::pyobject::PyRef;
use crate::pyobject::PyValue;
use crate::pyobject::{PyContext, PyObjectRef};
use crate::vm::VirtualMachine;
use core::cell::RefCell;

#[derive(Default, Debug)]
pub struct PyBytes {
    inner: RefCell<PyByteInner>,
}
pub type PyBytesRef = PyRef<PyBytes>;

#[derive(Default, Debug)]
pub struct PyByteArray {
    inner: PyByteInner,
}
pub type PyByteArrayRef = PyRef<PyByteArray>;

impl PyValue for PyBytes {
    fn class(vm: &VirtualMachine) -> PyClassRef {
        vm.ctx.bytes_type()
    }
}

impl PyValue for PyByteArray {
    fn class(vm: &VirtualMachine) -> PyClassRef {
        vm.ctx.bytearray_type()
    }
}

#[derive(Debug, Default, Clone)]
struct PyByteInner {
    elements: Vec<u8>,
}

// Fill bytes class methods:
pub fn bytes_init(context: &PyContext) {
    let bytes_type = context.bytes_type.as_object();

    let bytes_doc =
        "bytes(iterable_of_ints) -> bytes\n\
         bytes(string, encoding[, errors]) -> bytes\n\
         bytes(bytes_or_buffer) -> immutable copy of bytes_or_buffer\n\
         bytes(int) -> bytes object of size given by the parameter initialized with null bytes\n\
         bytes() -> empty bytes object\n\nConstruct an immutable array of bytes from:\n  \
         - an iterable yielding integers ibytes_doc56)\n  \
         - a text string encoded using the specified encoding\n  \
         - any object implementing the buffer API.\n  \
         - an integer";

    // extend_class!(context, bytes_type, {
    // "__new__" => context.new_rustfunc(bytes_new),
    // "__eq__" => context.new_rustfunc(PyBytesRef::eq),
    // "__lt__" => context.new_rustfunc(PyBytesRef::lt),
    // "__le__" => context.new_rustfunc(PyBytesRef::le),
    // "__gt__" => context.new_rustfunc(PyBytesRef::gt),
    // "__ge__" => context.new_rustfunc(PyBytesRef::ge),
    // "__hash__" => context.new_rustfunc(PyBytesRef::hash),
    // "__repr__" => context.new_rustfunc(PyBytesRef::repr),
    // "__len__" => context.new_rustfunc(PyBytesRef::len),
    // "__iter__" => context.new_rustfunc(PyBytesRef::iter),
    // "__doc__" => context.new_str(bytes_doc.to_string())
    // });

    // let bytesiterator_type = &context.bytesiterator_type;
    // extend_class!(context, bytesiterator_type, {
    //     "__next__" => context.new_rustfunc(PyBytesIteratorRef::next),
    //     "__iter__" => context.new_rustfunc(PyBytesIteratorRef::iter),
    // });
}

/// Fill bytearray class methods dictionary.
pub fn bytearray_init(context: &PyContext) {
    let bytearray_type = &context.bytearray_type;

    let bytearray_doc =
        "bytearray(iterable_of_ints) -> bytearray\n\
         bytearray(string, encoding[, errors]) -> bytearray\n\
         bytearray(bytes_or_buffer) -> mutable copy of bytes_or_buffer\n\
         bytearray(int) -> bytes array of size given by the parameter initialized with null bytes\n\
         bytearray() -> empty bytes array\n\n\
         Construct a mutable bytearray object from:\n  \
         - an iterable yielding integers in range(256)\n  \
         - a text string encoded using the specified encoding\n  \
         - a bytes or a buffer object\n  \
         - any object implementing the buffer API.\n  \
         - an integer";

    // extend_class!(context, bytearray_type, {
    // "__doc__" => context.new_str(bytearray_doc.to_string()),
    // "__new__" => context.new_rustfunc(bytearray_new),
    //     "__eq__" => context.new_rustfunc(PyByteArrayRef::eq),
    //     "__len__" => context.new_rustfunc(PyByteArrayRef::len),
    //     "__repr__" => context.new_rustfunc(PyByteArrayRef::repr),
    //     "__iter__" => context.new_rustfunc(PyByteArrayRef::iter),
    //     "clear" => context.new_rustfunc(PyByteArrayRef::clear),
    //     "isalnum" => context.new_rustfunc(PyByteArrayRef::isalnum),
    //     "isalpha" => context.new_rustfunc(PyByteArrayRef::isalpha),
    //     "isascii" => context.new_rustfunc(PyByteArrayRef::isascii),
    //     "isdigit" => context.new_rustfunc(PyByteArrayRef::isdigit),
    //     "islower" => context.new_rustfunc(PyByteArrayRef::islower),
    //     "isspace" => context.new_rustfunc(PyByteArrayRef::isspace),
    //     "istitle" =>context.new_rustfunc(PyByteArrayRef::istitle),
    //     "isupper" => context.new_rustfunc(PyByteArrayRef::isupper),
    //     "lower" => context.new_rustfunc(PyByteArrayRef::lower),
    //     "pop" => context.new_rustfunc(PyByteArrayRef::pop),
    //     "upper" => context.new_rustfunc(PyByteArrayRef::upper)
    // });

    // let bytearrayiterator_type = &context.bytearrayiterator_type;
    // extend_class!(context, bytearrayiterator_type, {
    //     "__next__" => context.new_rustfunc(PyByteArrayIteratorRef::next),
    //     "__iter__" => context.new_rustfunc(PyByteArrayIteratorRef::iter),
    // });
}
