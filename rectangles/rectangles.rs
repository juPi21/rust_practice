use std::collections::HashSet;

struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point, // верхня ліва точка
    b: Point, // нижня права точка
}

fn area_occupied(rectangles: &Vec<Rectangle>) -> i32 {
    let mut occupied_points: HashSet<(i32, i32)> = HashSet::new();

    for rect in rectangles {
        // Визначаємо координати
        let x_start = rect.a.x.min(rect.b.x);
        let x_end = rect.a.x.max(rect.b.x);
        let y_start = rect.a.y.max(rect.b.y); // верхня границя
        let y_end = rect.a.y.min(rect.b.y); // нижня границя

        // Перевірка правильності границь
        println!("Обробка прямокутника: ({}, {}) - ({}, {})", x_start, y_start, x_end, y_end);
        
        // Ітерація по точках
        for x in x_start..x_end {
            for y in y_end..y_start { // Порядок y: y_end до y_start
                occupied_points.insert((x, y));
                println!("Додаємо точку: ({}, {})", x, y);
            }
        }
    }

    // Повертаємо кількість зайнятих точок
    occupied_points.len() as i32
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 }, // верхня ліва
            b: Point { x: 5, y: 3 }, // нижня права
        },
        Rectangle {
            a: Point { x: 7, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 10, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 48); // Очікуване значення
}

fn main() {
    area_occupied_test();
    println!("Тест успішно пройдено!");
}
