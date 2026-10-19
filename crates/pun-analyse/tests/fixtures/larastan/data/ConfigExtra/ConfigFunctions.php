<?php

enum PunConfigEnum
{
    case A;
}

function pun_config_object(): \stdClass
{
    return new \stdClass();
}

function pun_config_enum(): PunConfigEnum
{
    return PunConfigEnum::A;
}

function pun_config_closure(): \Closure
{
    return fn () => 1;
}

/** @return callable(): string */
function pun_config_callable(): callable
{
    return fn () => 'x';
}

function pun_config_mixed(): mixed
{
    return 1;
}
