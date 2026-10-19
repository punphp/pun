<?php

try {
    a();
} catch (A $e) {
} catch (B|C $e) {
    ;
    {}
    { ; { } }
} catch (D $e) {
    // only a comment
} catch (E) {
    b();
} finally {
}

try {
} catch (F $e) {
    { c(); }
}
