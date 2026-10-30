<?php declare(strict_types=1);

namespace ParamsBranchContracts;

const LOCAL = 2;
class DeclaresConstant
{
    public static function declareConstant(): void
    {
        define('PARAMS_BRANCH_CLASS_CONSTANT', 2);
    }
}

function identities(): void
{
    echo json_encode([], LOCAL);
    echo json_encode([], JSON_BIGINT_AS_STRING);
    echo json_encode([], \true);
    echo json_encode([], \false);
    echo json_encode([], \null);
    echo json_encode([], \PARAMS_BRANCH_CLASS_CONSTANT);
    echo json_encode([], MISSING_PARAMS_BRANCH_CONSTANT | JSON_BIGINT_AS_STRING);
}
