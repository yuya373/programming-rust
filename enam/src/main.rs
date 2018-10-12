use std::cmp::Ordering;
use std::cmp::Ordering::*;

fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Less
    } else if n > m {
        Greater
    } else {
        Equal
    }
}

enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

fn http_status_from_u32(n: u32) -> Option<HttpStatus> {
    match n {
        200 => Some(HttpStatus::Ok),
        304 => Some(HttpStatus::NotModified),
        404 => Some(HttpStatus::NotFound),
        _ => None,
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    fn singular(self) -> &'static str {
        self.plural().trim_right_matches('s')
    }
}
#[derive(Debug)]
struct Point3d(f32, f32, f32);
enum Shape {
    Sphere { center: Point3d, radius: f32 },
    Cuboid { corner1: Point3d, corner2: Point3d },
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

impl RoughTime {
    pub fn to_string(self) -> String {
        match self {
            RoughTime::JustNow => String::from("just now"),

            RoughTime::InThePast(TimeUnit::Hours, 1) => format!("an hour ago"),
            RoughTime::InThePast(unit, 1) => format!("a {} ago", unit.singular()),
            RoughTime::InThePast(unit, duration) => format!("{} {} ago", duration, unit.plural()),
            RoughTime::InTheFuture(TimeUnit::Hours, 1) => format!("an hour from now"),
            RoughTime::InTheFuture(unit, 1) => format!("a {} from now", unit.singular()),
            RoughTime::InTheFuture(unit, duration) => {
                format!("{} {} from now", duration, unit.plural())
            }
        }
    }
}

struct DifferentialEquation;
struct EarlyModernistPoem;

enum RelationshipStatus {
    Single,
    InARelationship,
    ItsComplicated(Option<String>),
    ItsExtremelyComplicated {
        car: DifferentialEquation,
        cdr: EarlyModernistPoem,
    },
}

use std::collections::HashMap;
// json takes 4 words in memory.
// String and Vec takes 3 words, and tag take 1 byte.
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

fn main() {
    println!("Hello, world!");

    use std::mem::size_of;
    assert_eq!(size_of::<Ordering>(), 1);
    assert_eq!(size_of::<HttpStatus>(), 2);

    let four_score_and_seven_years_ago = RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);
    let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);
    println!("{}", four_score_and_seven_years_ago.to_string());
    println!("{}", three_hours_from_now.to_string());

    let one_year_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 1);
    println!("{}", one_year_from_now.to_string());

    let point = Point3d(1.0, 1.0, 0.0);
    let sphere = Shape::Sphere {
        center: point,
        radius: 8.0,
    };
    // sphere.center; // can't access with dot.
    // use pattern match
    match sphere {
        Shape::Sphere { center, radius } => println!("center: {:?}, radius: {}", center, radius),
        _ => println!("Cuboid is ignored"),
    }

    let array_in_json = Json::Array(vec![
        Json::String("foo".to_string()),
        Json::String("bar".to_string()),
        Json::String("baz".to_string()),
        Json::Null,
        Json::Number(2.0),
        Json::Boolean(true),
        Json::Object(Box::new(HashMap::new())),
    ]);

    use self::BinaryTree::*;

    let jupiter = TreeNode {
        element: "Jupiter",
        left: Empty,
        right: Empty,
    };
    let jupiter_tree = NonEmpty(Box::new(jupiter));

    let mercury = TreeNode {
        element: "Mercury",
        left: Empty,
        right: Empty,
    };
    let mercury_tree = NonEmpty(Box::new(mercury));

    let mars = TreeNode {
        element: "Mars",
        left: jupiter_tree,
        right: mercury_tree,
    };
    let mars_tree = NonEmpty(Box::new(mars));

    let venus = TreeNode {
        element: "Venus",
        left: Empty,
        right: Empty,
    };
    let venus_tree = NonEmpty(Box::new(venus));

    let uranus = TreeNode {
        element: "Uranus",
        left: Empty,
        right: venus_tree,
    };
    let uranus_tree = NonEmpty(Box::new(uranus));

    let saturn = TreeNode {
        element: "Saturn",
        left: mars_tree,
        right: uranus_tree,
    };
    let saturn_tree = NonEmpty(Box::new(saturn));

    println!("binray tree: {:?}", saturn_tree);
}
