use polars::prelude::*;
use polars_core::prelude::*;
use polars_lazy::{ prelude::*, dsl::Expr };
use data_science::datasets;
fn main() {
    // let df = get_wifi_localization_df();
    let df = datasets::get_gender_height_weight_df();
    println!("{:#?}", df.head(Some(10)));
    let col0 = df.column("Gender").unwrap();
    let gini = gini_index(col0);
    println!("gini index : {}", gini);
    let mut min_gini = (1f32, 1i32);
    for h in (100..250).step_by(2) {
        println!("height : {}", h);
        let tall_filter = df["Weight"].gt(h).unwrap();
        let small_filter = df["Weight"].lt(h).unwrap();
        let df_tall = df.filter(&tall_filter).unwrap();
        let df_small = df.filter(&small_filter).unwrap();
        let gini_small = gini_index(df_small.column("Gender").unwrap());
        let gini_tall = gini_index(df_tall.column("Gender").unwrap());
        // println!("gini small : {}", gini_small);
        // println!("gini tall : {}", gini_tall);
        let gini_sum =
            ((df_tall.shape().0 as f32) * gini_tall + (df_small.shape().0 as f32) * gini_small) /
            (df.shape().0 as f32);
        println!("gini sum : {}", gini_sum);
        if gini_sum < min_gini.0 {
            min_gini = (gini_sum, h);
        }
        println!("min : {:#?}", min_gini);
    }
}

pub fn gini_index(y: &Series) -> f32 {
    /*
  Given a Pandas Series, it calculates the Gini Impurity. 
  y: variable with which calculate Gini Impurity.
  */

    // comptage des valeurs différentes dans la série
    let vc = y.value_counts(false, false).unwrap();
    println!("vc : {}", vc);

    // longueur de la série
    let heigth: f32 = y.len() as f32;

    // on cherche la répartition de chaque valeur
    let percent = vc
        .lazy()
        .with_column((col("counts") / lit(heigth)).alias("percent"))
        .collect()
        .unwrap();
    println!("répartition percent {:#?}", percent);
    /*
    répartition p2 Ok(
    shape: (2, 3)
    ┌────────┬────────┬─────┐
    │ Gender ┆ counts ┆ percent│
    │ ---    ┆ ---    ┆ --- │
    │ str    ┆ u32    ┆ f64 │
    ╞════════╪════════╪═════╡
    │ Female ┆ 5000   ┆ 0.5 │
    │ Male   ┆ 5000   ┆ 0.5 │
    └────────┴────────┴─────┘,
)*/
    // on peut calculer l'index Gini (= 1 - (p1^2 + p2^2 + ....))
    let squared_percents = percent.column("percent").unwrap() * percent.column("percent").unwrap();
    let gini = 1_f32 - squared_percents.sum::<f32>().unwrap();
    return gini;
}

fn best_split(df: DataFrame, feature: &str) {
    let df = df.column(feature).unwrap().unique().unwrap();

    println!("itération");
    println!("{}", df)
    // .collect::<Float32Chunked>();
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_gini_index_gender() {
        let df = datasets::get_gender_height_weight_df();
        println!("{:#?}", df.head(Some(10)));
        let col0 = df.column("Gender").unwrap();
        let gini = gini_index(col0);
        println!("gini index gender : {}", gini);
    }
    #[test]
    fn test_gini_index_wifi_loc() {
        let df = datasets::get_wifi_localization_df();
        println!("{:#?}", df.head(Some(10)));
        let col0 = df.column("room").unwrap();
        let gini = gini_index(col0);
        println!("gini index wifi: {}", gini);
    }
    #[test]
    fn test_best_split() {
        let df = datasets::get_gender_height_weight_df();
        // println!("{:#?}", df.head(Some(10)));
        best_split(df, "Height")
    }
}