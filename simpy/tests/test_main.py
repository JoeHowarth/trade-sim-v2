from hello import sum_as_string, foo

def test_main():
    assert sum_as_string(2, 2) == '4'

    assert foo(2) == 4

