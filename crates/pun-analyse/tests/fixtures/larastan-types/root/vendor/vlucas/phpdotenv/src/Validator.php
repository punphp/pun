<?php

declare(strict_types=1);

namespace Dotenv;

use Dotenv\Exception\ValidationException;
use Dotenv\Repository\RepositoryInterface;
use Dotenv\Util\Regex;
use Dotenv\Util\Str;

class Validator
{
    /**
     * The environment repository instance.
     *
     * @var \Dotenv\Repository\RepositoryInterface
     */
    private $repository;

    /**
     * The variables to validate.
     *
     * @var string[]
     */
    private $variables;

    /**
     * Create a new validator instance.
     *
     * @param \Dotenv\Repository\RepositoryInterface $repository
     * @param string[]                               $variables
     *
     * @return void
     */
    public function __construct(RepositoryInterface $repository, array $variables)
    {}

    /**
     * Assert that each variable is present.
     *
     * @throws \Dotenv\Exception\ValidationException
     *
     * @return \Dotenv\Validator
     */
    public function required()
    {}

    /**
     * Assert that each variable is not empty.
     *
     * @throws \Dotenv\Exception\ValidationException
     *
     * @return \Dotenv\Validator
     */
    public function notEmpty()
    {}

    /**
     * Assert that each specified variable is an integer.
     *
     * @throws \Dotenv\Exception\ValidationException
     *
     * @return \Dotenv\Validator
     */
    public function isInteger()
    {}

    /**
     * Assert that each specified variable is a boolean.
     *
     * @throws \Dotenv\Exception\ValidationException
     *
     * @return \Dotenv\Validator
     */
    public function isBoolean()
    {}

    /**
     * Assert that each variable is amongst the given choices.
     *
     * @param string[] $choices
     *
     * @throws \Dotenv\Exception\ValidationException
     *
     * @return \Dotenv\Validator
     */
    public function allowedValues(array $choices)
    {}

    /**
     * Assert that each variable matches the given regular expression.
     *
     * @param string $regex
     *
     * @throws \Dotenv\Exception\ValidationException
     *
     * @return \Dotenv\Validator
     */
    public function allowedRegexValues(string $regex)
    {}

    /**
     * Assert that the callback returns true for each variable.
     *
     * @param callable(?string):bool $callback
     * @param string                 $message
     *
     * @throws \Dotenv\Exception\ValidationException
     *
     * @return \Dotenv\Validator
     */
    public function assert(callable $callback, string $message)
    {}

    /**
     * Assert that the callback returns true for each variable.
     *
     * Skip checking null variable values.
     *
     * @param callable(string):bool $callback
     * @param string                $message
     *
     * @throws \Dotenv\Exception\ValidationException
     *
     * @return \Dotenv\Validator
     */
    public function assertNullable(callable $callback, string $message)
    {}
}
