(SELECT count() as count FROM documents GROUP ALL)[0].count;
DELETE FROM documents;
SELECT * FROM documents WHERE content IS NOT NULL LIMIT 1;

SELECT content FROM documents WHERE string::contains(content, "carta dirigida a um organ");
SELECT * OMIT embedding FROM documents ORDER BY metadata.source, metadata.index;
SELECT * FROM documents;