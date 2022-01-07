BEGIN;

DROP TABLE IF EXISTS movies; 

CREATE TABLE movies (
    id SERIAL NOT NULL PRIMARY KEY,
    title text NOT NULL,
    poster_url text
);

-- Seed db with test data
INSERT INTO movies
(title, poster_url)
VALUES
('Tommy Boy', 'http://image.tmdb.org/t/p/original/g32WbO9nbY5ydpux5hIoiJkLEQi.jpg'),
('Beauty and the Beast', 'https://image.tmdb.org/t/p/original/fVf2YzecYSjY19hXkd2RhrEL7mI.jpg');

COMMIT;