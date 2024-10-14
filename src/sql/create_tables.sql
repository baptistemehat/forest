-- tree table stores trees
CREATE TABLE IF NOT EXISTS tree (
    name TEXT NOT NULL PRIMARY KEY,
    description TEXT NOT NULL
);

-- task table stores tasks
CREATE TABLE IF NOT EXISTS task (
    id TEXT NOT NULL PRIMARY KEY,
    tree_name TEXT NOT NULL,
    "left" INTEGER NOT NULL,
    "right" INTEGER NOT NULL,
    name TEXT NOT NULL,
    description text NOT NULL,
    FOREIGN KEY (tree_name) REFERENCES tree(name) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS state (
    current_tree TEXT DEFAULT NULL,
    FOREIGN KEY (current_tree) REFERENCES tree(name) ON UPDATE CASCADE ON DELETE
    SET
        DEFAULT
);

-- insert initial empty current_tree
INSERT INTO state("current_tree")
SELECT NULL
WHERE NOT EXISTS (SELECT * FROM state);


-- -- frame table store time frames
-- CREATE table if not EXISTS frame (
-- 	id integer primary key,
-- 	start integer not null,
-- 	stop integer,
-- 	task integer not null,
-- 	foreign key (task) references task(id)
-- );
