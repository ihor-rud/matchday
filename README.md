Simple program for fetching top scored and assisted players using Sportradar API.

Use this command to run the program:

```bash
cargo run -- --api-token <token>
```

If you are using trial api key then you need to add addition argument:

```bash
cargo run -- --api-token <token> --enable-rate-limit
```

Check `--help` output for other arguments:

```bash
cargo run -- --help
```

Example output:

```
Top assisted players:
+-------------------+---------------------+------------------+------------------------+---------+
| id                | name                | team_id          | team_name              | assists |
+-------------------+---------------------+------------------+------------------------+---------+
| sr:player:555386  | Watkins, Ollie      | sr:competitor:40 | Aston Villa            | 13      |
+-------------------+---------------------+------------------+------------------------+---------+
| sr:player:1737609 | Palmer, Cole        | sr:competitor:38 | Chelsea FC             | 11      |
+-------------------+---------------------+------------------+------------------------+---------+
| sr:player:1089760 | Gibbs-White, Morgan | sr:competitor:14 | Nottingham Forest      | 10      |
+-------------------+---------------------+------------------+------------------------+---------+
| sr:player:70996   | De Bruyne, Kevin    | sr:competitor:17 | Manchester City        | 10      |
+-------------------+---------------------+------------------+------------------------+---------+
| sr:player:48480   | Gross, Pascal       | sr:competitor:30 | Brighton & Hove Albion | 10      |
+-------------------+---------------------+------------------+------------------------+---------+
| sr:player:111505  | Heung-min, Son      | sr:competitor:33 | Tottenham Hotspur      | 10      |
+-------------------+---------------------+------------------+------------------------+---------+
| sr:player:1646334 | Johnson, Brennan    | sr:competitor:33 | Tottenham Hotspur      | 10      |
+-------------------+---------------------+------------------+------------------------+---------+
| sr:player:69256   | Trippier, Kieran    | sr:competitor:39 | Newcastle United       | 10      |
+-------------------+---------------------+------------------+------------------------+---------+
| sr:player:1373595 | Gordon, Anthony     | sr:competitor:39 | Newcastle United       | 10      |
+-------------------+---------------------+------------------+------------------------+---------+
| sr:player:883594  | Bailey, Leon        | sr:competitor:40 | Aston Villa            | 10      |
+-------------------+---------------------+------------------+------------------------+---------+
Top scored players:
+-------------------+-----------------------+------------------+-------------------+--------+
| id                | name                  | team_id          | team_name         | scores |
+-------------------+-----------------------+------------------+-------------------+--------+
| sr:player:991181  | Haaland, Erling       | sr:competitor:17 | Manchester City   | 27     |
+-------------------+-----------------------+------------------+-------------------+--------+
| sr:player:1737609 | Palmer, Cole          | sr:competitor:38 | Chelsea FC        | 22     |
+-------------------+-----------------------+------------------+-------------------+--------+
| sr:player:925724  | Isak, Alexander       | sr:competitor:39 | Newcastle United  | 21     |
+-------------------+-----------------------+------------------+-------------------+--------+
| sr:player:1047129 | Foden, Phil           | sr:competitor:17 | Manchester City   | 19     |
+-------------------+-----------------------+------------------+-------------------+--------+
| sr:player:555386  | Watkins, Ollie        | sr:competitor:40 | Aston Villa       | 19     |
+-------------------+-----------------------+------------------+-------------------+--------+
| sr:player:361420  | Solanke, Dominic      | sr:competitor:60 | AFC Bournemouth   | 19     |
+-------------------+-----------------------+------------------+-------------------+--------+
| sr:player:159665  | Salah, Mohamed        | sr:competitor:44 | Liverpool FC      | 18     |
+-------------------+-----------------------+------------------+-------------------+--------+
| sr:player:111505  | Heung-min, Son        | sr:competitor:33 | Tottenham Hotspur | 17     |
+-------------------+-----------------------+------------------+-------------------+--------+
| sr:player:1027583 | Mateta, Jean-Philippe | sr:competitor:7  | Crystal Palace    | 16     |
+-------------------+-----------------------+------------------+-------------------+--------+
| sr:player:552884  | Bowen, Jarrod         | sr:competitor:37 | West Ham United   | 16     |
+-------------------+-----------------------+------------------+-------------------+--------+
```
