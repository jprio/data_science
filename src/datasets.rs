use polars::prelude::*;
use std::io::{ Cursor };
use reqwest::blocking::get;

pub fn get_wifi_localization_df() -> DataFrame {
    let cursor = get_cursor(
        "https://archive.ics.uci.edu/ml/machine-learning-databases/00422/wifi_localization.txt"
    );
    let mut df = match CsvReader::new(cursor).with_delimiter(b'\t').has_header(false).finish() {
        Err(why) => panic!("{:?}", why),
        Ok(df) => df,
    };
    df.set_column_names(
        &["wifi1", "wifi2", "wifi3", "wifi4", "wifi5", "wifi6", "wifi7", "room"]
    ).unwrap();
    df
}

pub fn get_gender_height_weight_df() -> DataFrame {
    let cursor = get_cursor(
        "https://gist.githubusercontent.com/nstokoe/7d4717e96c21b8ad04ec91f361b000cb/raw/bf95a2e30fceb9f2ae990eac8379fc7d844a0196/weight-height.csv"
    );
    let df = match CsvReader::new(cursor).with_delimiter(b',').has_header(true).finish() {
        Err(why) => panic!("{:?}", why),
        Ok(df) => df,
    };
    df
}
pub fn get_banknote_df() -> DataFrame {
    /*Below provides a list of the five variables in the dataset.
        variance of Wavelet Transformed image (continuous).
        skewness of Wavelet Transformed image (continuous).
        kurtosis of Wavelet Transformed image (continuous).
        entropy of image (continuous).
        class (integer).
     */
    let cursor = get_cursor(
        "http://archive.ics.uci.edu/ml/machine-learning-databases/00267/data_banknote_authentication.txt"
    );
    let df = match CsvReader::new(cursor).with_delimiter(b',').has_header(true).finish() {
        Err(why) => panic!("{:?}", why),
        Ok(df) => df,
    };
    df
}
pub fn get_gender_height_weight_imc_df() -> DataFrame {
    let cursor = get_cursor(
        "https://raw.githubusercontent.com/chriswmann/datasets/master/500_Person_Gender_Height_Weight_Index.csv"
    );
    let df = match CsvReader::new(cursor).with_delimiter(b',').has_header(true).finish() {
        Err(why) => panic!("{:?}", why),
        Ok(df) => df,
    };
    df
}

pub fn get_cursor(target: &str) -> Cursor<Vec<u8>> {
    let response = get(target).unwrap().text().unwrap();
    let cursor = Cursor::new(response.into_bytes());
    cursor
}