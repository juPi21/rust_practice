fn main() {
  let digits = [1, 2, 3, 4, 5, 6, 7, 8];
  let mut solutions = vec![];

  for &m in &digits {
      for &u in &digits {
          if u == m { continue; }
          for &x in &digits {
              if x == m || x == u { continue; }
              for &a in &digits {
                  if a == m || a == u || a == x { continue; }
                  for &s in &digits {
                      if s == m || s == u || s == x || s == a { continue; }
                      for &l in &digits {
                          if l == m || l == u || l == x || l == a || l == s { continue; }
                          for &o in &digits {
                              if o == m || o == u || o == x || o == a || o == s || o == l { continue; }
                              for &n in &digits {
                                  if n == m || n == u || n == x || n == a || n == s || n == l || n == o { continue; }

                                  let muxa = 1000 * m + 100 * u + 10 * x + a;
                                  let slon = 1000 * s + 100 * l + 10 * o + n;

                                  if muxa * a == slon {
                                      solutions.push((muxa, a, slon));
                                  }
                              }
                          }
                      }
                  }
              }
          }
      }
  }

  for (muxa, a, slon) in &solutions {
      println!("{} x {} = {}", muxa, a, slon);
  }
  println!("Кількість рішень: {}", solutions.len());
}
