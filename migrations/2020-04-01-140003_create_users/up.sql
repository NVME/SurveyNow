
/**https://www.postgresqltutorial.com/postgresql-identity-column/**/

CREATE TABLE users (
  id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  email VARCHAR(100) NOT NULL ,
  username VARCHAR(100) NOT NULL,
  password VARCHAR(64) NOT NULL,
  created_at TIMESTAMP NOT NULL
);

