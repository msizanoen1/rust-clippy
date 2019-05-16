#![allow(default_hash_types, non_upper_case_globals)]

use lazy_static::lazy_static;
use syntax::symbol::{Interner, Symbol};
// this import is very important! otherwise we may redefine symbols that already exist in rustc
// and thus cause two equal symbols to differ
pub use syntax::symbol::sym::*;

macro_rules! symbols_simple {
    ($($ident:ident,)*) => {
        pub const PREINTERNED_SYMBOLS: &[&str] = &[
            "<impl [T]>",
            $(stringify!($ident)),*
        ];

        lazy_static! {
            pub(crate) static ref impl_slice_t: Symbol = {
                let sym = Symbol::intern("<impl [T]>");
                assert_eq!(sym.as_u32(), Interner::FIRST_DRIVER_INDEX);
                sym
            };
        }
        $(
            lazy_static! {
                // we can keep using `from_str` here, because it'll fetch the preinterned symbol id
                // and that will be stable across reruns of clippy even if the global table gets
                // nuked.
                pub(crate) static ref $ident: Symbol = Symbol::intern(stringify!($ident));
            }
        )*
    };
}

// exists because concat_idents is flaky
pub mod assign {
    pub(crate) use super::AddAssign as Add;
    pub(crate) use super::AndAssign as And;
    pub(crate) use super::BitAndAssign as BitAnd;
    pub(crate) use super::BitOrAssign as BitOr;
    pub(crate) use super::BitXorAssign as BitXor;
    pub(crate) use super::DivAssign as Div;
    pub(crate) use super::MulAssign as Mul;
    pub(crate) use super::OrAssign as Or;
    pub(crate) use super::RemAssign as Rem;
    pub(crate) use super::ShlAssign as Shl;
    pub(crate) use super::ShrAssign as Shr;
    pub(crate) use super::SubAssign as Sub;
}

symbols_simple! {
    Option,
    rustc,
    AsMut,
    AsRef,
    Clone,
    Default,
    DoubleEndedIterator,
    Drop,
    From,
    Into,
    IntoIterator,
    Iterator,
    Ord,
    PartialOrd,
    Any,
    add,
    Add,
    AddAssign,
    AndAssign,
    OrAssign,
    all,
    alloc,
    always,
    any,
    Arc,
    Arguments,
    array,
    as_bytes,
    as_mut,
    as_ref,
    assert,
    as_str,
    automatically_derived,
    begin_panic,
    begin_panic_fmt,
    binary_heap,
    BinaryHeap,
    bitand,
    BitAndAssign,
    bitor,
    BitOrAssign,
    bitxor,
    BitXorAssign,
    bool,
    borrow,
    Borrow,
    borrow_mut,
    btree,
    BTreeMap,
    BTreeSet,
    by_ref,
    bytes,
    capacity,
    cfg,
    cfg_attr,
    chain,
    chars,
    clone,
    cloned,
    cmp,
    collect,
    collections,
    conf_file,
    contains,
    contains_key,
    context,
    convert,
    core,
    count,
    Cow,
    c_str,
    CString,
    cycle,
    dbg,
    de,
    debug_assert,
    default,
    deprecated,
    deref,
    Deref,
    deref_mut,
    discriminant,
    Display,
    div,
    Div,
    DivAssign,
    doc,
    drop,
    Duration,
    E,
    EarlyContext,
    end,
    ends_with,
    Entry,
    enumerate,
    eq,
    Err,
    extend,
    ffi,
    filter,
    filter_map,
    find,
    flat_map,
    fmt,
    fold,
    for_each,
    forget,
    format,
    FRAC_1_PI,
    FRAC_1_SQRT_2,
    FRAC_2_PI,
    FRAC_2_SQRT_PI,
    FRAC_PI_2,
    FRAC_PI_3,
    FRAC_PI_4,
    FRAC_PI_6,
    FRAC_PI_8,
    from,
    from_elem,
    from_iter,
    from_str,
    fs,
    fuse,
    hash,
    Hash,
    HashMap,
    HashSet,
    hidden,
    i128,
    i16,
    i32,
    i64,
    i8,
    Implied,
    index,
    Index,
    index_mut,
    IndexMut,
    init,
    inline,
    insert,
    inspect,
    into_iter,
    into_result,
    into_vec,
    intrinsics,
    io,
    is_empty,
    is_err,
    isize,
    is_none,
    is_ok,
    is_some,
    iter,
    Iter,
    iterator,
    iter_mut,
    last,
    LateContext,
    len,
    linked_list,
    LinkedList,
    lint,
    Lint,
    LintPass,
    LN_10,
    LN_2,
    LOG10_E,
    LOG2_E,
    macro_use,
    main,
    map,
    matches,
    match_indices,
    max,
    MAX,
    max_by,
    max_by_key,
    mem,
    min,
    MIN,
    min_by,
    min_by_key,
    mpsc,
    mul,
    Mul,
    MulAssign,
    mutex,
    Mutex,
    NAN,
    ne,
    neg,
    new,
    new_v1,
    new_v1_formatted,
    next,
    next_back,
    None,
    not,
    null,
    null_mut,
    offset,
    ok,
    Ok,
    ONCE_INIT,
    open,
    OpenOptions,
    ops,
    option,
    os_str,
    OsStr,
    OsString,
    panic,
    panicking,
    partition,
    path,
    Path,
    PathBuf,
    paths,
    peekable,
    PI,
    position,
    precision,
    print,
    println,
    proc_macro,
    proc_macro_attribute,
    proc_macro_derive,
    product,
    ptr,
    push,
    Range,
    RangeBounds,
    RangeFrom,
    RangeFull,
    RangeInclusive,
    RangeTo,
    RangeToInclusive,
    rc,
    Rc,
    Read,
    re_builder,
    re_bytes,
    Receiver,
    regex,
    Regex,
    RegexBuilder,
    RegexSet,
    rem,
    RemAssign,
    repeat,
    replace,
    re_set,
    resize,
    result,
    Result,
    re_unicode,
    rev,
    rfind,
    rmatches,
    rmatch_indices,
    rplit_terminator,
    rposition,
    rsplit,
    rsplitn,
    rsplit_terminator,
    rustfmt,
    rustfmt_skip,
    scan,
    serde,
    set,
    shl,
    ShlAssign,
    shr,
    ShrAssign,
    since,
    skip,
    skip_while,
    slice,
    Some,
    split,
    splitn,
    split_terminator,
    SQRT_2,
    start,
    starts_with,
    std,
    stderr,
    stdin,
    stdio,
    stdout,
    string,
    String,
    sub,
    Sub,
    SubAssign,
    sum,
    sync,
    take,
    take_while,
    test,
    time,
    to_os_string,
    to_owned,
    ToOwned,
    to_path_buf,
    to_string,
    ToString,
    traits,
    transmute,
    trim_end_matches,
    trim_start_matches,
    Try,
    u128,
    u16,
    u32,
    u64,
    u8,
    unicode,
    unimplemented,
    uninit,
    uninitialized,
    unreachable,
    unused_extern_crates,
    unused_imports,
    unwrap,
    unwrap_err,
    unzip,
    usize,
    utils,
    vec,
    Vec,
    vec_deque,
    VecDeque,
    Visitor,
    Weak,
    width,
    with_capacity,
    wrapping_offset,
    write,
    Write,
    write_fmt,
    writeln,
    zeroed,
    zip,
}
