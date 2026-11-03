<?php

namespace Illuminate\Console;

use InvalidArgumentException;
use Symfony\Component\Console\Input\InputArgument;
use Symfony\Component\Console\Input\InputOption;

class Parser
{
    /**
     * Parse the given console command definition into an array.
     *
     * @param  string  $expression
     * @return array{string, array{}, array{}}|array{string, \Symfony\Component\Console\Input\InputArgument[], \Symfony\Component\Console\Input\InputOption[]}
     *
     * @throws \InvalidArgumentException
     */
    public static function parse(string $expression)
    {}

    /**
     * Extract the name of the command from the expression.
     *
     * @param  string  $expression
     * @return string
     *
     * @throws \InvalidArgumentException
     */
    protected static function name(string $expression)
    {}

    /**
     * Extract all parameters from the tokens.
     *
     * @param  string[]  $tokens
     * @return array{\Symfony\Component\Console\Input\InputArgument[], \Symfony\Component\Console\Input\InputOption[]}
     */
    protected static function parameters(array $tokens)
    {}

    /**
     * Parse an argument expression.
     *
     * @param  string  $token
     * @return \Symfony\Component\Console\Input\InputArgument
     */
    protected static function parseArgument(string $token)
    {}

    /**
     * Parse an option expression.
     *
     * @param  string  $token
     * @return \Symfony\Component\Console\Input\InputOption
     */
    protected static function parseOption(string $token)
    {}

    /**
     * Parse the token into its token and description segments.
     *
     * @param  string  $token
     * @return array{string, string}
     */
    protected static function extractDescription(string $token)
    {}
}
