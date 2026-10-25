<?php
namespace AttributePre80;
#[\Attribute(\Attribute::IS_REPEATABLE | \Attribute::TARGET_FUNCTION)]
class Arguments
{
    /** @no-named-arguments */
    public function __construct(int $number, string &$text) {}
}
#[Arguments(number: 1, text: 'valid')]
#[Arguments(number: 'wrong', text: 123)]
function accepted(): void {}
#[Arguments(number: 1)]
function missing(): void {}
#[Arguments(unknown: 1)]
function unknown(): void {}
