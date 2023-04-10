CREATE TABLE todos (
  id SERIAL PRIMARY KEY,
  description VARCHAR NOT NULL,
  completed BOOLEAN NOT NULL,
  userId INT NOT NULL,
  dateCreated TIMESTAMP WITH TIME ZONE NOT NULL,
  dateUpdated TIMESTAMP WITH TIME ZONE NULL
);

ALTER TABLE todos ADD CONSTRAINT todos_userId_fkey FOREIGN KEY (userId) REFERENCES users(id) ON DELETE RESTRICT ON UPDATE CASCADE;