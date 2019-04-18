from testutils import assertRaises

# new
assert bytes([1, 2, 3])
assert bytes((1, 2, 3))
assert bytes(range(4))
assert bytes(3)
assert b"bla"
assert bytes("bla", "utf8")
with assertRaises(TypeError):
    bytes("bla")


a = b"abcd"
b = b"ab"
c = b"abcd"

#
# repr
assert repr(bytes([0, 1, 2])) == repr(b"\x00\x01\x02")
assert repr(
    bytes([0, 1, 9, 10, 11, 13, 31, 32, 33, 89, 120, 255])
    == "b'\\x00\\x01\\t\\n\\x0b\\r\\x1f !Yx\\xff'"
)
assert repr(b"abcd") == "b'abcd'"

# len
assert len(bytes("abcdé", "utf8")) == 6

# comp
assert a == b"abcd"
assert a > b
assert a >= b
assert b < a
assert b <= a

assert b"foobar".__eq__(2) == NotImplemented
assert b"foobar".__ne__(2) == NotImplemented
assert b"foobar".__gt__(2) == NotImplemented
assert b"foobar".__ge__(2) == NotImplemented
assert b"foobar".__lt__(2) == NotImplemented
assert b"foobar".__le__(2) == NotImplemented

# hash
hash(a) == hash(b"abcd")

# iter
[i for i in b"abcd"] == ["a", "b", "c", "d"]
assert list(bytes(3)) == [0, 0, 0]

# add
assert a + b == b"abcdab"

# contains
assert b"ab" in b"abcd"
assert b"cd" in b"abcd"
assert b"abcd" in b"abcd"
assert b"a" in b"abcd"
assert b"d" in b"abcd"
assert b"dc" not in b"abcd"
assert 97 in b"abcd"
assert 150 not in b"abcd"
with assertRaises(ValueError):
    350 in b"abcd"


# getitem
d = b"abcdefghij"

assert d[1] == 98
assert d[-1] == 106
assert d[2:6] == b"cdef"
assert d[-6:] == b"efghij"
assert d[1:8:2] == b"bdfh"
assert d[8:1:-2] == b"igec"


# is_xx methods

assert bytes(b"1a23").isalnum()
assert not bytes(b"1%a23").isalnum()

assert bytes(b"abc").isalpha()
assert not bytes(b"abc1").isalpha()

# travis doesn't like this
# assert bytes(b'xyz').isascii()
# assert not bytes([128, 157, 32]).isascii()

assert bytes(b"1234567890").isdigit()
assert not bytes(b"12ab").isdigit()

l = bytes(b"lower")
b = bytes(b"UPPER")

assert l.islower()
assert not l.isupper()
assert b.isupper()
assert not bytes(b"Super Friends").islower()

assert bytes(b" \n\t").isspace()
assert not bytes(b"\td\n").isspace()

assert b.isupper()
assert not b.islower()
assert l.islower()
assert not bytes(b"tuPpEr").isupper()

assert bytes(b"Is Title Case").istitle()
assert not bytes(b"is Not title casE").istitle()

# upper lower, capitalize
l = bytes(b"lower")
b = bytes(b"UPPER")
assert l.lower().islower()
assert b.upper().isupper()
assert l.capitalize() == b"Lower"
assert b.capitalize() == b"Upper"
assert bytes().capitalize() == bytes()

# hex from hex
assert bytes([0, 1, 9, 23, 90, 234]).hex() == "000109175aea"

bytes.fromhex("62 6c7a 34350a ") == b"blz45\n"
try:
    bytes.fromhex("62 a 21")
except ValueError as e:
    str(e) == "non-hexadecimal number found in fromhex() arg at position 4"
try:
    bytes.fromhex("6Z2")
except ValueError as e:
    str(e) == "non-hexadecimal number found in fromhex() arg at position 1"

# center
assert [b"koki".center(i, b"|") for i in range(3, 10)] == [
    b"koki",
    b"koki",
    b"|koki",
    b"|koki|",
    b"||koki|",
    b"||koki||",
    b"|||koki||",
]

assert [b"kok".center(i, b"|") for i in range(2, 10)] == [
    b"kok",
    b"kok",
    b"kok|",
    b"|kok|",
    b"|kok||",
    b"||kok||",
    b"||kok|||",
    b"|||kok|||",
]
b"kok".center(4) == b" kok"  # " test no arg"
with assertRaises(TypeError):
    b"b".center(2, "a")
with assertRaises(TypeError):
    b"b".center(2, b"ba")
b"kok".center(5, bytearray(b"x"))
b"kok".center(-5)

# count
assert b"azeazerazeazopia".count(b"aze") == 3
assert b"azeazerazeazopia".count(b"az") == 4
assert b"azeazerazeazopia".count(b"a") == 5
assert b"123456789".count(b"") == 10
assert b"azeazerazeazopia".count(bytearray(b"aze")) == 3
assert b"azeazerazeazopia".count(memoryview(b"aze")) == 3
assert b"azeazerazeazopia".count(memoryview(b"aze"), 1, 9) == 1
assert b"azeazerazeazopia".count(b"aze", None, None) == 3
assert b"azeazerazeazopia".count(b"aze", 2, None) == 2
assert b"azeazerazeazopia".count(b"aze", 2) == 2
assert b"azeazerazeazopia".count(b"aze", None, 7) == 2
assert b"azeazerazeazopia".count(b"aze", None, 7) == 2
assert b"azeazerazeazopia".count(b"aze", 2, 7) == 1
assert b"azeazerazeazopia".count(b"aze", -13, -10) == 1
assert b"azeazerazeazopia".count(b"aze", 1, 10000) == 2
with assertRaises(ValueError):
    b"ilj".count(3550)
assert b"azeazerazeazopia".count(97) == 5

# join
assert (
    b"".join((b"jiljl", bytearray(b"kmoomk"), memoryview(b"aaaa")))
    == b"jiljlkmoomkaaaa"
)
with assertRaises(TypeError):
    b"".join((b"km", "kl"))


# endswith startswith
assert b"abcde".endswith(b"de")
assert b"abcde".endswith(b"")
assert not b"abcde".endswith(b"zx")
assert b"abcde".endswith(b"bc", 0, 3)
assert not b"abcde".endswith(b"bc", 2, 3)
assert b"abcde".endswith((b"c", b"de"))

assert b"abcde".startswith(b"ab")
assert b"abcde".startswith(b"")
assert not b"abcde".startswith(b"zx")
assert b"abcde".startswith(b"cd", 2)
assert not b"abcde".startswith(b"cd", 1, 4)
assert b"abcde".startswith((b"a", b"bc"))


# index find
assert b"abcd".index(b"cd") == 2
assert b"abcd".index(b"cd", 0) == 2
assert b"abcd".index(b"cd", 1) == 2
assert b"abcd".index(99) == 2
with assertRaises(ValueError):
    b"abcde".index(b"c", 3, 1)
with assertRaises(ValueError):
    b"abcd".index(b"cdaaaaa")
with assertRaises(ValueError):
    b"abcd".index(b"b", 3, 4)
with assertRaises(ValueError):
    b"abcd".index(1)


assert b"abcd".find(b"cd") == 2
assert b"abcd".find(b"cd", 0) == 2
assert b"abcd".find(b"cd", 1) == 2
assert b"abcde".find(b"c", 3, 1) == -1
assert b"abcd".find(b"cdaaaaa") == -1
assert b"abcd".find(b"b", 3, 4) == -1
assert b"abcd".find(1) == -1
assert b"abcd".find(99) == 2


# make trans
# fmt: off
assert ( 
    bytes.maketrans(memoryview(b"abc"), bytearray(b"zzz"))
    == bytes([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 122, 122, 122, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255])
)
# fmt: on

# translate
assert b'hjhtuyjyujuyj'.translate(bytes.maketrans(b"hj", b"ab"), b"h") == b'btuybyubuyb'
assert b'hjhtuyjyujuyj'.translate(bytes.maketrans(b"hj", b"ab"), b"a") == b'abatuybyubuyb'
assert b'hjhtuyjyujuyj'.translate(bytes.maketrans(b"hj", b"ab")) == b'abatuybyubuyb'
assert b'hjhtuyfjtyhuhjuyj'.translate(None, b"ht") == b'juyfjyujuyj'

