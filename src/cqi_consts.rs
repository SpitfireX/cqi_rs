//
//   CQi v0.1
//   (IMS/HIT Corpus Query Interface)
//   (C) Stefan Evert, IMS Stuttgart, Sep 1999
//


// default port for CQi services
pub const PORT: u16 = 4877;
// padding
pub const PAD: u8 =  0x00;

//  ***
//  ***   CQi responses
//  ***

#[derive(Debug, enum_utils::FromStr, num_derive::FromPrimitive)]
#[repr(u8)]
pub enum ResponseType {
    STATUS = 0x01,
    ERROR = 0x02,
    DATA = 0x03,
    CL_ERROR = 0x04,
    CQP_ERROR = 0x05,
}

#[derive(Debug, enum_utils::FromStr, num_derive::FromPrimitive)]
#[repr(u16)]
pub enum STATUS {
    OK = 0x0101,
    CONNECT_OK = 0x0102,
    BYE_OK = 0x0103,
    PING_OK = 0x0104,
}

#[derive(Debug, enum_utils::FromStr, num_derive::FromPrimitive)]
#[repr(u16)]
pub enum ERROR {
    GENERAL_ERROR = 0x0201,
    CONNECT_REFUSED = 0x0202,
    USER_ABORT = 0x0203,
    SYNTAX_ERROR = 0x0204,
    // includes corpus/attribute/subcorpus specifier syntax
}

#[derive(Debug, enum_utils::FromStr, num_derive::FromPrimitive)]
#[repr(u16)]
pub enum DATA {
    BYTE = 0x0301,
    BOOL = 0x0302,
    INT = 0x0303,
    STRING = 0x0304,
    BYTE_LIST = 0x0305,
    BOOL_LIST = 0x0306,
    INT_LIST = 0x0307,
    STRING_LIST = 0x0308,
    INT_INT = 0x0309,
    INT_INT_INT_INT = 0x030A,
    INT_TABLE = 0x030B,
}

#[derive(Debug, enum_utils::FromStr, num_derive::FromPrimitive)]
#[repr(u16)]
pub enum CL_ERROR {
    NO_SUCH_ATTRIBUTE = 0x0401,
    // returned if CQi server couldn't open attribute
    
    WRONG_ATTRIBUTE_TYPE = 0x0402,
    // CDA_EATTTYPE
    
    OUT_OF_RANGE = 0x0403,
    // CDA_EIDORNG, CDA_EIDXORNG, CDA_EPOSORNG
    
    REGEX = 0x0404,
    // CDA_EPATTERN (not used), CDA_EBADREGEX
    
    CORPUS_ACCESS = 0x0405,
    // CDA_ENODATA
    
    OUT_OF_MEMORY = 0x0406,
    // CDA_ENOMEM
    // this means the CQi server has run out of memory;
    // try discarding some other corpora and/or subcorpora
}

#[derive(Debug, enum_utils::FromStr, num_derive::FromPrimitive)]
#[repr(u16)]
pub enum CQP_ERROR {
    GENERAL = 0x0501,
    NO_SUCH_CORPUS = 0x0502,
    INVALID_FIELD = 0x0503,
    OUT_OF_RANGE = 0x0504,
}

//  ***
//  ***   CQi commands
//  ***

#[derive(Debug, enum_utils::FromStr, num_derive::FromPrimitive)]
#[repr(u16)]
pub enum COMMANDS {
    // CTRL = 0x1100,

    CTRL_CONNECT = 0x1101,
    // INPUT: (STRING username, STRING password)
    // OUTPUT: STATUS::CONNECT_OK, ERROR::CONNECT_REFUSED

    CTRL_BYE = 0x1102,
    // INPUT: ()
    // OUTPUT: STATUS::BYE_OK

    CTRL_USER_ABORT = 0x1103,
    // INPUT: ()
    // OUTPUT:

    CTRL_PING = 0x1104,
    // INPUT: ()
    // OUTPUT: STATUS::PING_OK

    CTRL_LAST_GENERAL_ERROR = 0x1105,
    // INPUT: ()
    // OUTPUT: DATA::STRING
    // full-text error message for the last general error reported by
    // the CQi server


    // ASK_FEATURE = 0x1200,

    ASK_FEATURE_CQI_1_0 = 0x1201,
    // INPUT: ()
    // OUTPUT: DATA::BOOL

    ASK_FEATURE_CL_2_3 = 0x1202,
    // INPUT: ()
    // OUTPUT: DATA::BOOL

    ASK_FEATURE_CQP_2_3 = 0x1203,
    // INPUT: ()
    // OUTPUT: DATA::BOOL


    // CORPUS = 0x1300,

    CORPUS_LIST_CORPORA = 0x1301,
    // INPUT: ()
    // OUTPUT: DATA::STRING_LIST

    CORPUS_CHARSET = 0x1303,
    // INPUT: (STRING corpus)
    // OUTPUT: DATA::STRING

    CORPUS_PROPERTIES = 0x1304,
    // INPUT: (STRING corpus)
    // OUTPUT: DATA::STRING_LIST

    CORPUS_POSITIONAL_ATTRIBUTES = 0x1305,
    // INPUT: (STRING corpus)
    // OUTPUT: DATA::STRING_LIST

    CORPUS_STRUCTURAL_ATTRIBUTES = 0x1306,
    // INPUT: (STRING corpus)
    // OUTPUT: DATA::STRING_LIST

    CORPUS_STRUCTURAL_ATTRIBUTE_HAS_VALUES = 0x1307,
    // INPUT: (STRING attribute)
    // OUTPUT: DATA::BOOL

    CORPUS_ALIGNMENT_ATTRIBUTES = 0x1308,
    // INPUT: (STRING corpus)
    // OUTPUT: DATA::STRING_LIST

    CORPUS_FULL_NAME = 0x1309,
    // INPUT: (STRING corpus)
    // OUTPUT: DATA::STRING
    // the full name of <corpus> as specified in its registry entry

    CORPUS_INFO = 0x130A,
    // INPUT: (STRING corpus)
    // OUTPUT: DATA::STRING_LIST
    // returns the contents of the .info file of <corpus> as a list of lines

    CORPUS_DROP_CORPUS = 0x130B,
    // INPUT: (STRING corpus)
    // OUTPUT: STATUS::OK
    // try to unload a corpus and all its attributes from memory


    // CL = 0x1400,
    // low-level corpus access (CL functions)

    CL_ATTRIBUTE_SIZE = 0x1401,
    // INPUT: (STRING attribute)
    // OUTPUT: DATA::INT
    // returns the size of <attribute>:
    //     number of tokens        (positional)
    //     number of regions       (structural)
    //     number of alignments    (alignment)

    CL_LEXICON_SIZE = 0x1402,
    // INPUT: (STRING attribute)
    // OUTPUT: DATA::INT
    // returns the number of entries in the lexicon of a positional attribute;
    // valid lexicon IDs range from 0 .. (lexicon_size - 1)

    CL_DROP_ATTRIBUTE = 0x1403,
    // INPUT: (STRING attribute)
    // OUTPUT: STATUS::OK
    // unload attribute from memory

    CL_STR2ID = 0x1404,
    // INPUT: (STRING attribute, STRING_LIST strings)
    // OUTPUT: DATA::INT_LIST
    // returns -1 for every string in <strings> that is not found in the lexicon

    CL_ID2STR = 0x1405,
    // INPUT: (STRING attribute, INT_LIST id)
    // OUTPUT: DATA::STRING_LIST
    // returns "" for every ID in <id> that is out of range

    CL_ID2FREQ = 0x1406,
    // INPUT: (STRING attribute, INT_LIST id)
    // OUTPUT: DATA::INT_LIST
    // returns 0 for every ID in <id> that is out of range

    CL_CPOS2ID = 0x1407,
    // INPUT: (STRING attribute, INT_LIST cpos)
    // OUTPUT: DATA::INT_LIST
    // returns -1 for every corpus position in <cpos> that is out of range

    CL_CPOS2STR = 0x1408,
    // INPUT: (STRING attribute, INT_LIST cpos)
    // OUTPUT: DATA::STRING_LIST
    // returns "" for every corpus position in <cpos> that is out of range

    CL_CPOS2STRUC = 0x1409,
    // INPUT: (STRING attribute, INT_LIST cpos)
    // OUTPUT: DATA::INT_LIST
    // returns -1 for every corpus position not inside a structure region

    // temporary addition for the Euralex2000 tutorial, but should probably be included in CQi specs
    CL_CPOS2LBOUND = 0x1420,
    // INPUT: (STRING attribute, INT_LIST cpos)
    // OUTPUT: DATA::INT_LIST
    // returns left boundary of s-attribute region enclosing cpos, -1 if not in region

    CL_CPOS2RBOUND = 0x1421,
    // INPUT: (STRING attribute, INT_LIST cpos)
    // OUTPUT: DATA::INT_LIST
    // returns right boundary of s-attribute region enclosing cpos, -1 if not in region

    CL_CPOS2ALG = 0x140A,
    // INPUT: (STRING attribute, INT_LIST cpos)
    // OUTPUT: DATA::INT_LIST
    // returns -1 for every corpus position not inside an alignment

    CL_STRUC2STR = 0x140B,
    // INPUT: (STRING attribute, INT_LIST strucs)
    // OUTPUT: DATA::STRING_LIST
    // returns annotated string values of structure regions in <strucs>; "" if out of range
    // check COMMANDS::CORPUS_STRUCTURAL_ATTRIBUTE_HAS_VALUES(<attribute>) first

    CL_ID2CPOS = 0x140C,
    // INPUT: (STRING attribute, INT id)
    // OUTPUT: DATA::INT_LIST
    // returns all corpus positions where the given token occurs

    CL_IDLIST2CPOS = 0x140D,
    // INPUT: (STRING attribute, INT_LIST id_list)
    // OUTPUT: DATA::INT_LIST
    // returns all corpus positions where one of the tokens in <id_list>
    // occurs; the returned list is sorted as a whole, not per token id

    CL_REGEX2ID = 0x140E,
    // INPUT: (STRING attribute, STRING regex)
    // OUTPUT: DATA::INT_LIST
    // returns lexicon IDs of all tokens that match <regex>; the returned
    // list may be empty (size 0);

    CL_STRUC2CPOS = 0x140F,
    // INPUT: (STRING attribute, INT struc)
    // OUTPUT: DATA::INT_INT
    // returns start and end corpus positions of structure region <struc>

    CL_ALG2CPOS = 0x1410,
    // INPUT: (STRING attribute, INT alg)
    // OUTPUT: DATA::INT_INT_INT_INT
    // returns (src_start, src_end, target_start, target_end)


    // CQP = 0x1500,

    CQP_QUERY = 0x1501,
    // INPUT: (STRING mother_corpus, STRING subcorpus_name, STRING query)
    // OUTPUT: STATUS::OK
    // <query> must include the ';' character terminating the query.

    CQP_LIST_SUBCORPORA = 0x1502,
    // INPUT: (STRING corpus)
    // OUTPUT: DATA::STRING_LIST

    CQP_SUBCORPUS_SIZE = 0x1503,
    // INPUT: (STRING subcorpus)
    // OUTPUT: DATA::INT

    CQP_SUBCORPUS_HAS_FIELD = 0x1504,
    // INPUT: (STRING subcorpus, BYTE field)
    // OUTPUT: DATA::BOOL

    CQP_DUMP_SUBCORPUS = 0x1505,
    // INPUT: (STRING subcorpus, BYTE field, INT first, INT last)
    // OUTPUT: DATA::INT_LIST
    // Dump the values of <field> for match ranges <first> .. <last>
    // in <subcorpus>. <field> is one of the FIELD_* constants.

    CQP_DROP_SUBCORPUS = 0x1509,
    // INPUT: (STRING subcorpus)
    // OUTPUT: STATUS::OK
    // delete a subcorpus from memory

    // The following two functions are temporarily included for the Euralex 2000 tutorial demo
    // frequency distribution of single tokens
    CQP_FDIST_1 = 0x1510,
    // INPUT: (STRING subcorpus, INT cutoff, BYTE field, STRING attribute)
    // OUTPUT: DATA::INT_LIST
    // returns <n> (id, frequency) pairs flattened into a list of size 2*<n>
    // field is one of FIELD_MATCH, FIELD_TARGET, FIELD_KEYWORD
    // NB: pairs are sorted by frequency desc.

    // frequency distribution of pairs of tokens
    CQP_FDIST_2 = 0x1511,
    // INPUT: (STRING subcorpus, INT cutoff, BYTE field1, STRING attribute1, BYTE field2, STRING attribute2)
    // OUTPUT: DATA::INT_LIST
    // returns <n> (id1, id2, frequency) pairs flattened into a list of size 3*<n>
    // NB: triples are sorted by frequency desc.
}

//  ***
//  ***   Constant Definitions
//  ***

pub const FALSE: u8 = 0x00;
pub const NO: u8 = 0x00;

pub const TRUE: u8 = 0x01;
pub const YES: u8 = 0x01;

// The following constants specify which field will be returned
// by CQI_CQP_DUMP_SUBCORPUS and some other subcorpus commands.

pub const FIELD_MATCH: u8 = 0x10;
pub const FIELD_MATCHEND: u8 = 0x11;

// The constants specifiying target0 .. target9 are guaranteed to
// have the numerical values 0 .. 9, so clients do not need to look
// up the constant values if they're handling arbitrary targets.
pub const FIELD_TARGET_0: u8 = 0x00;
pub const FIELD_TARGET_1: u8 = 0x01;
pub const FIELD_TARGET_2: u8 = 0x02;
pub const FIELD_TARGET_3: u8 = 0x03;
pub const FIELD_TARGET_4: u8 = 0x04;
pub const FIELD_TARGET_5: u8 = 0x05;
pub const FIELD_TARGET_6: u8 = 0x06;
pub const FIELD_TARGET_7: u8 = 0x07;
pub const FIELD_TARGET_8: u8 = 0x08;
pub const FIELD_TARGET_9: u8 = 0x09;

// The following constants are provided for backward compatibility
// with traditional CQP field names & while the generalised target
// concept isn't yet implemented in the CQPserver.
pub const FIELD_TARGET: u8 = 0x00;
pub const FIELD_KEYWORD: u8 = 0x09;


// CQi version is CQI_MAJOR_VERSION.CQI_MINOR_VERSION
pub const CQI_MAJOR_VERSION: u8 = 0x00;
pub const CQI_MINOR_VERSION: u8 = 0x01;