.open test.db

CREATE TABLE scores (
    player TEXT,
    game TEXT,
    score INTEGER,
    score_date DATETIME
);

INSERT INTO scores (player, game, score, score_date)
VALUES
    ('Billy Mitchell', 'Pac-Man', 3333360, '1999-07-03'),
    ('Steve Wiebe', 'Donkey Kong', 1064500, '2007-03-06'),
    ('Robbie Lakeman', 'Donkey Kong', 1141800, '2014-06-09'),
    ('George Leutz', 'Q*bert', 37163080, '2013-02-18'),
    ('Hank Chien', 'Donkey Kong', 994400, '2011-03-19'),
    ('Scott Safran', 'Asteroids', 41336440, '1982-11-13'),
    ('Robbie Lakeman', 'Donkey Kong', 1272800, '2021-06-06');
