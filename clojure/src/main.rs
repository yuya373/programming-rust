fn main() {
    println!("Hello, world!");

    struct City {
        name: String,
        population: i64,
        country: String,
        monster_attack_risk: f64,
    }

    // fn sort_cities(cities: &mut Vec<City>) {
    //     cities.sort(); // std::cmp::Ord is not implemented for City
    // }

    fn city_population_descending(city: &City) -> i64 {
        -city.population
    }

    fn sort_cities(cities: &mut Vec<City>) {
        // cities.sort_by_key(city_population_descending);
        cities.sort_by_key(|city| -city.population);
    }

    struct Statistic;

    impl City {
        pub fn get_statistic(&self, _stat: &Statistic) -> i64 {
            9
        }
    }

    fn sort_by_statistic(cities: &mut Vec<City>, stat: Statistic) {
        cities.sort_by_key(|city| -city.get_statistic(&stat));
    }

    use std::thread;

    fn start_sorting_thread(
        mut cities: Vec<City>,
        stat: Statistic,
    ) -> thread::JoinHandle<Vec<City>> {
        let key_fn = move |city: &City| -> i64 { -city.get_statistic(&stat) };
        thread::spawn(|| {
            cities.sort_by_key(key_fn);
            cities
        })
    }

    let key_fn: fn(&City) -> i64 = city_population_descending;

    fn count_selected_cities(cities: &Vec<City>, test_fn: fn(&City) -> bool) -> usize {
        let mut count = 0;
        for city in cities {
            if test_fn(city) {
                count += 1;
            }
        }
        count
    }

    fn has_monster_attacks(city: &City) -> bool {
        city.monster_attack_risk > 0.0
    }

    let my_cities = Vec::new();
    let n = count_selected_cities(&my_cities, has_monster_attacks);

    let n = count_selected_cities(&my_cities, |city| city.monster_attack_risk > 0.0);

    fn count_selected_cities_with_clojure<F>(cities: &Vec<City>, test_fn: F) -> usize
    where
        F: Fn(&City) -> bool,
    {
        let mut count = 0;
        for city in cities {
            if test_fn(city) {
                count += 1;
            }
        }
        count
    }

    let my_str = "hello".to_string();
    let f = || drop(my_str);
    f();
    // f(); // use of moved value `f`

    fn call_twice<F>(closure: F)
    where
        F: Fn() -> (),
        // F: Fn(),
    {
        closure();
        closure();
    }

    let my_str = "hello".to_string();
    // expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
    // let f = || drop(my_str); // FnOnce
    // call_twice(f);

    use std::collections::HashMap;
    let dict: HashMap<String, i64> = HashMap::new();
    let debug_dump_dict = || {
        // consume `dict`
        for (key, value) in dict {
            println!("{:?} - {:?}", key, value);
        }
    }; // FnOnce

    debug_dump_dict();
    // debug_dump_dict(); // use of moved value

    let dict: HashMap<String, i64> = HashMap::new();
    let debug_dump_dict = || {
        // borrow `dict` instead of move
        for (key, value) in &dict {
            println!("{:?} - {:?}", key, value);
        }
    }; // Fn

    debug_dump_dict();
    debug_dump_dict();

    let mut i = 0;
    let incr = || {
        i += 1;
        println!("Ding! i is now: {}", i);
    }; // FnMut

    // call_twice(incr); // incr does not impl `Fn`

    fn call_twice_mut<F>(mut closure: F)
    where
        F: FnMut() -> (),
    {
        closure();
        closure();
    }

    call_twice_mut(incr);

    let mut i = 0;
    call_twice_mut(|| i += 1);
    assert_eq!(i, 2);

    struct Request {
        method: String,
        url: String,
        headers: HashMap<String, String>,
        body: Vec<u8>,
    }

    struct Response {
        code: u32,
        headers: HashMap<String, String>,
        body: Vec<u8>,
    }

    impl Response {
        fn new() -> Response {
            Response {
                code: 200,
                headers: HashMap::new(),
                body: Vec::new(),
            }
        }
    }

    struct BasicRouter<F>
    where
        F: Fn(&Request) -> Response,
    {
        routes: HashMap<String, F>,
    }

    impl<F> BasicRouter<F>
    where
        F: Fn(&Request) -> Response,
    {
        fn new() -> BasicRouter<F> {
            BasicRouter {
                routes: HashMap::new(),
            }
        }

        fn add_route(&mut self, url: &str, callback: F) {
            self.routes.insert(url.to_string(), callback);
        }
    }

    let mut router = BasicRouter::new();
    router.add_route("/", |_| Response::new());

    // no two closures, even if identical, have the same type [E0308]
    // consider boxing your closure and/or using it as a trait object [E0308]
    // router.add_route("/gcd", |_| Response::new());

    type BoxedF = Box<Fn(&Request) -> Response>;
    struct BoxedBasicRouter {
        routes: HashMap<String, BoxedF>,
    }

    impl BoxedBasicRouter {
        fn new() -> BoxedBasicRouter {
            BoxedBasicRouter {
                routes: HashMap::new(),
            }
        }

        fn add_route<F>(&mut self, url: &str, callback: F)
        where
            F: Fn(&Request) -> Response + 'static,
        {
            self.routes.insert(url.to_string(), Box::new(callback));
        }
    }

    let mut router = BoxedBasicRouter::new();
    router.add_route("/", |_| Response::new());
    router.add_route("/gcd", |_| Response::new());
}
