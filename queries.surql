(SELECT count() as count FROM documents GROUP ALL)[0].count;
DELETE FROM documents;
SELECT * FROM documents;