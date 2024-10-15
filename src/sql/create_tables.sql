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

-- state table stores current state
CREATE TABLE IF NOT EXISTS state (
    current_tree TEXT DEFAULT NULL,
    FOREIGN KEY (current_tree) REFERENCES tree(name) ON UPDATE CASCADE ON DELETE
    SET
        DEFAULT
);

-- insert initial empty current_tree
INSERT INTO
    state("current_tree")
SELECT
    NULL
WHERE
    NOT EXISTS (
        SELECT
            *
        FROM
            state
    );

-- frame table stores time frames
CREATE TABLE IF NOT EXISTS frame (
    id text NOT NULL PRIMARY KEY,
    "start" integer NOT NULL,
	"end" integer,
    task_id text NOT NULL,
    FOREIGN KEY (task_id) REFERENCES task(id) ON DELETE CASCADE
);
