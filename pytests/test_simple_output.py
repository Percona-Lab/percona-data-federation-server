import pytest
from test_util.pdfs import Pdfs


def test_output():
    p = Pdfs('4', '5')
    p.start()
    line = p.process.stdout.readline()
    assert line.rstrip() == '{"value":29}'
    p.stop()
