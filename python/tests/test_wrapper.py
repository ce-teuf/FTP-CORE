
import pytest

# Ajouter le chemin du projet au sys.path
#sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))

from ..src.ftp_core import add, safe_divide, MonWrapper, batch_process

def test_add():
    assert add(2, 3) == 5
    assert add(-1, 1) == 0

def test_safe_divide():
    assert safe_divide(10, 2) == 5.0
    assert safe_divide(10, 0) == float('inf')

def test_mon_wrapper():
    wrapper = MonWrapper()
    assert wrapper.fonction_etendue(5) == 15

    with pytest.raises(ValueError):
        wrapper.fonction_etendue(-1)

def test_batch_process():
    assert batch_process([1, 2, 3]) == [2, 3, 4]

