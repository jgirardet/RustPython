from testutils import assertRaises

# ut8

invalid = bytes([169, 195, 169, 97, 101, 169, 169, 195, 169, 105, 169, 195, 169])
valid = bytes([195, 169, 97, 101, 195, 169, 105, 195, 169])
with assertRaises(UnicodeError):
    invalid.decode()
with assertRaises(UnicodeError):
    invalid.decode("utf8")
with assertRaises(UnicodeError):
    invalid.decode("utf8", "strict")
with assertRaises(LookupError):
    invalid.decode("azd")
assert invalid.decode(errors="replace") == "�éae��éi�é"
assert invalid.decode(errors="ignore") == "éaeéié"
assert invalid.decode(errors="backslashreplace") == r"\xa9éae\xa9\xa9éi\xa9é"
assert valid.decode() == "éaeéié"
