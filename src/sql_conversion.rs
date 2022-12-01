use std::fs::File;
use std::io::{BufReader, BufRead, Error};

pub(crate) struct FictionSql(pub(crate) File);

impl FictionSql {
    pub(crate) fn get_tables_script(&self) -> Vec<&'static str> {
        vec![DROP_FICTION, CREATE_FICTION]
    }

    pub(crate) fn get_rows_insert_scripts(&self) -> impl Iterator<Item = Result<String, Error>>+'_ {
        BufReader::new(&self.0)
            .lines()
            .filter(|line_read_result| line_read_result.is_ok() &&  { 
                let line: &String = line_read_result.as_ref().unwrap();
                line.trim_start().to_lowercase().starts_with("insert")
            })
    }
}

const DROP_FICTION: &str = "DROP TABLE IF EXISTS public.fiction;";
const CREATE_FICTION: &str = "";

/* FICTION.SQL

DROP TABLE IF EXISTS `fiction`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `fiction` (
  `ID` int(15) unsigned NOT NULL AUTO_INCREMENT,
  `MD5` char(32) CHARACTER SET ascii DEFAULT NULL,
  `Title` varchar(2000) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `Author` varchar(300) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `Series` varchar(300) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `Edition` varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `Language` varchar(45) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `Year` varchar(10) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `Publisher` varchar(100) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `Pages` varchar(10) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `Identifier` varchar(400) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `GooglebookID` varchar(45) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `ASIN` varchar(10) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `Coverurl` varchar(200) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `Extension` varchar(10) COLLATE utf8mb4_unicode_ci NOT NULL,
  `Filesize` int(10) unsigned NOT NULL,
  `Library` varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `Issue` varchar(100) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `Locator` varchar(512) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `Commentary` varchar(500) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `Generic` char(32) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `Visible` char(3) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `TimeAdded` timestamp NOT NULL DEFAULT current_timestamp(),
  `TimeLastModified` timestamp NULL DEFAULT NULL ON UPDATE current_timestamp(),
  PRIMARY KEY (`ID`),
  UNIQUE KEY `MD5UNIQUE` (`MD5`) USING BTREE,
  KEY `Language` (`Language`),
  FULLTEXT KEY `TITLE` (`Title`),
  FULLTEXT KEY `Authors` (`Author`),
  FULLTEXT KEY `Series` (`Series`),
  FULLTEXT KEY `Title+Authors+Series` (`Title`,`Author`,`Series`),
  FULLTEXT KEY `Identifier` (`Identifier`)
) ENGINE=MyISAM AUTO_INCREMENT=2540050 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

DROP TABLE IF EXISTS `fiction_description`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `fiction_description` (
  `MD5` char(32) CHARACTER SET ascii NOT NULL,
  `Descr` mediumtext COLLATE utf8mb4_bin NOT NULL,
  `TimeLastModified` timestamp NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),
  PRIMARY KEY (`MD5`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;

DROP TABLE IF EXISTS `fiction_hashes`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `fiction_hashes` (
  `md5` char(32) NOT NULL,
  `crc32` char(8) NOT NULL DEFAULT '',
  `edonkey` char(32) NOT NULL DEFAULT '',
  `aich` char(32) NOT NULL DEFAULT '',
  `sha1` char(40) NOT NULL DEFAULT '',
  `tth` char(39) NOT NULL DEFAULT '',
  `btih` char(40) NOT NULL DEFAULT '',
  `sha256` char(64) NOT NULL DEFAULT '',
  `ipfs_cid` char(62) NOT NULL DEFAULT '',
  PRIMARY KEY (`md5`)
) ENGINE=MyISAM DEFAULT CHARSET=ascii;


*/