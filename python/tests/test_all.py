import pytest
import scan_search


def test_sum_as_string():
    assert scan_search.sum_as_string(1, 1) == "2"
