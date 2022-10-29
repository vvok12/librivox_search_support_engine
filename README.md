# librivox_search_support_engine
Full-text search in text sources of the books presented on LibriVox. Simply because their on-page search I don't like)

Here is an idea and some useful tools I've already collected:

1) Get books from the LibriVox library.
API: https://librivox.org/api/info
Example: https://github.com/kaveet/libri-node/blob/master/index.js

2) Get their text sources.
Although the LibriVox page usually contains links to some useful sources, they are not unified.
The proposition is to search for each book in the Library Genesis database.

There is a regularly updated libgen database dump collection: https://www.libgen.is/dbdumps/
This data is saved in MySql RDBMS format.
To unpack RAR: https://github.com/muja/unrar.rs

Also, there exists some Open Library. Consider it: https://openlibrary.org/.

Downloading the data from libgen mirrors requires some work. There is a Desktop Libgen app to take useful links: https://github.com/libgenapps/LibgenDesktop/blob/master/LibgenDesktop/Resources/Mirrors/mirrors.config
The most promising way to do it is using the "https://libgen.rocks/ads.php?md5={}" pattern, and then scanning the HTML response for the GET link.

3) Performing text search respectful to the user input mistakes over title and author. 
This could be done with a PostgreSQL trigram search. Move dump data to PostgreSQL.

4) If the text source is not in plain text format, convert it to plain text.
Fb2 could require zip-archiver

5) Performing full-text search. Learn: https://www.postgresql.org/docs/current/textsearch.html

6) Make the whole thing work)
