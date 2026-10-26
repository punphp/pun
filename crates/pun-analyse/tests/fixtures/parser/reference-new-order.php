<?php
$outer
    =& new class(
        $inner =& new stdClass
    ) {
        function parameter(
            void $value
        ) {}
        #[\self] function invalidName() {}
    };
namespace After {}
