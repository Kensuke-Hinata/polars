# flake8: noqa: W191,E101
from os import path
from pathlib import Path

import numpy as np

import polars as pl


def test_scan_csv() -> None:
    df = pl.scan_csv(Path(__file__).parent.parent / "files" / "small.csv")
    assert df.collect().shape == (4, 3)


def test_invalid_utf8() -> None:
    np.random.seed(1)
    bts = bytes(np.random.randint(0, 255, 200))
    file = path.join(path.dirname(__file__), "nonutf8.csv")

    with open(file, "wb") as f:
        f.write(bts)

    a = pl.read_csv(file, has_headers=False, encoding="utf8-lossy")
    b = pl.scan_csv(file, has_headers=False, encoding="utf8-lossy").collect()
    assert a.frame_equal(b, null_equal=True)


def test_row_count(foods_csv: str) -> None:
    df = pl.read_csv(foods_csv, row_count_name="row_count")
    assert df["row_count"].to_list() == list(range(27))

    df = (
        pl.scan_csv(foods_csv, row_count_name="row_count")
        .filter(pl.col("category") == pl.lit("vegetables"))
        .collect()
    )

    assert df["row_count"].to_list() == [0, 6, 11, 13, 14, 20, 25]

    df = (
        pl.scan_csv(foods_csv, row_count_name="row_count")
        .with_row_count("foo", 10)
        .filter(pl.col("category") == pl.lit("vegetables"))
        .collect()
    )

    assert df["foo"].to_list() == [10, 16, 21, 23, 24, 30, 35]
