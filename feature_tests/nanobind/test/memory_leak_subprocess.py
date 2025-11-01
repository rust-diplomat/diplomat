import somelib

a = somelib.OpaqueMutexedString.from_usize(0)
a.borrow()
b = somelib.OpaqueMutexedString.from_usize(0)
a.borrow_self_or_other(b)

f = somelib.Float64Vec.new([0, 1, 2])
f.borrow()