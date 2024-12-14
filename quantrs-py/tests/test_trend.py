import numpy as np
import pytest

from quantrs import sma


def test_sma_basic():
    data = [1.0, 2.0, 3.0, 4.0, 5.0]
    result = sma(data, 3)

    # Check type
    assert isinstance(result, np.ndarray)

    # Check values - first two should be NaN, then moving averages
    expected = np.array([np.nan, np.nan, 2.0, 3.0, 4.0])
    np.testing.assert_array_almost_equal(result, expected)


def test_sma_constant():
    data = [2.0, 2.0, 2.0, 2.0, 2.0]
    result = sma(data, 3)

    # All values after initial NaNs should be 2.0
    expected = np.array([np.nan, np.nan, 2.0, 2.0, 2.0])
    np.testing.assert_array_almost_equal(result, expected)


def test_sma_period_one():
    data = [1.0, 2.0, 3.0]
    result = sma(data, 1)

    # Period of 1 should return the same values
    np.testing.assert_array_almost_equal(result, data)


def test_sma_period_equals_length():
    data = [1.0, 2.0, 3.0, 4.0]
    result = sma(data, 4)

    # Only last value should be non-NaN
    expected = np.array([np.nan, np.nan, np.nan, 2.5])
    np.testing.assert_array_almost_equal(result, expected)


def test_sma_empty():
    result = sma([], 3)
    assert len(result) == 0


def test_sma_invalid_period():
    data = [1.0, 2.0, 3.0]

    with pytest.raises(ValueError):
        sma(data, 0)

    with pytest.raises(ValueError):
        sma(data, -1)


def test_sma_period_larger_than_data():
    data = [1.0, 2.0, 3.0]
    result = sma(data, 4)

    # All values should be NaN
    assert np.all(np.isnan(result))
    assert len(result) == len(data)
