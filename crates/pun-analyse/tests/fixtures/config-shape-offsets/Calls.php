<?php

namespace ConfigShapeOffsets;

use Illuminate\Support\Facades\Config;

function offsets(): void
{
    Config::collection('offsets.0');
    Config::collection('offsets.1');
    Config::collection('offsets.5');
    Config::collection('offsets.6');
    Config::collection('offsets.7');
    Config::collection('offsets.named');
    Config::collection('offsets.nested.0');
    Config::collection('offsets.nested.1');
    Config::collection('offsets.nested.4');
    Config::collection('offsets.nested.5');
    Config::collection('offsets.optional.0');
    Config::collection('offsets.optional.1');
    Config::collection('offsets.constants.0');
    Config::collection('offsets.constants.1');
    Config::collection('offsets.constants.2');
    Config::collection('offsets.namedConstant.0');
    Config::collection('offsets.namedConstant.Y-m-d\\TH:i:sP');
    Config::collection('offsets.optionalInteger.4');
    Config::collection('offsets.optionalInteger.5');
    Config::collection('offsets.optionalInteger.6');
    Config::collection('offsets.optionalInteger.tail');
    Config::collection('offsets.negativeInteger.-3');
    Config::collection('offsets.negativeInteger.0');
    Config::collection('offsets.negativeInteger.1');
    Config::collection('offsets.quotedInteger.5');
    Config::collection('offsets.quotedInteger.6');
    Config::collection('offsets.duplicate.0');
    Config::collection('offsets.duplicate.1');
    Config::collection('offsets.missingConstant.0');
    Config::collection('offsets.missingConstant.known');
}
