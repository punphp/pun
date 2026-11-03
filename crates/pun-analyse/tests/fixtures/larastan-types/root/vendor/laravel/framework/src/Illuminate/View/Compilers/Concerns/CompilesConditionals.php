<?php

namespace Illuminate\View\Compilers\Concerns;

use Illuminate\Support\Str;

trait CompilesConditionals
{
    /**
     * Identifier for the first case in the switch statement.
     *
     * @var bool
     */
    protected $firstCaseInSwitch = true;

    /**
     * Compile the if-auth statements into valid PHP.
     *
     * @param  string|null  $guard
     * @return string
     */
    protected function compileAuth($guard = null)
    {}

    /**
     * Compile the else-auth statements into valid PHP.
     *
     * @param  string|null  $guard
     * @return string
     */
    protected function compileElseAuth($guard = null)
    {}

    /**
     * Compile the end-auth statements into valid PHP.
     *
     * @return string
     */
    protected function compileEndAuth()
    {}

    /**
     * Compile the env statements into valid PHP.
     *
     * @param  string  $environments
     * @return string
     */
    protected function compileEnv($environments)
    {}

    /**
     * Compile the end-env statements into valid PHP.
     *
     * @return string
     */
    protected function compileEndEnv()
    {}

    /**
     * Compile the production statements into valid PHP.
     *
     * @return string
     */
    protected function compileProduction()
    {}

    /**
     * Compile the end-production statements into valid PHP.
     *
     * @return string
     */
    protected function compileEndProduction()
    {}

    /**
     * Compile the if-guest statements into valid PHP.
     *
     * @param  string|null  $guard
     * @return string
     */
    protected function compileGuest($guard = null)
    {}

    /**
     * Compile the else-guest statements into valid PHP.
     *
     * @param  string|null  $guard
     * @return string
     */
    protected function compileElseGuest($guard = null)
    {}

    /**
     * Compile the end-guest statements into valid PHP.
     *
     * @return string
     */
    protected function compileEndGuest()
    {}

    /**
     * Compile the has-section statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileHasSection($expression)
    {}

    /**
     * Compile the has-stack statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileHasStack($expression)
    {}

    /**
     * Compile the section-missing statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileSectionMissing($expression)
    {}

    /**
     * Compile the if statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileIf($expression)
    {}

    /**
     * Compile the unless statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileUnless($expression)
    {}

    /**
     * Compile the else-if statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileElseif($expression)
    {}

    /**
     * Compile the else statements into valid PHP.
     *
     * @return string
     */
    protected function compileElse()
    {}

    /**
     * Compile the end-if statements into valid PHP.
     *
     * @return string
     */
    protected function compileEndif()
    {}

    /**
     * Compile the end-unless statements into valid PHP.
     *
     * @return string
     */
    protected function compileEndunless()
    {}

    /**
     * Compile the if-isset statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileIsset($expression)
    {}

    /**
     * Compile the end-isset statements into valid PHP.
     *
     * @return string
     */
    protected function compileEndIsset()
    {}

    /**
     * Compile the switch statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileSwitch($expression)
    {}

    /**
     * Compile the case statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileCase($expression)
    {}

    /**
     * Compile the default statements in switch case into valid PHP.
     *
     * @return string
     */
    protected function compileDefault()
    {}

    /**
     * Compile the end switch statements into valid PHP.
     *
     * @return string
     */
    protected function compileEndSwitch()
    {}

    /**
     * Compile a once block into valid PHP.
     *
     * @param  string|null  $id
     * @return string
     */
    protected function compileOnce($id = null)
    {}

    /**
     * Compile an end-once block into valid PHP.
     *
     * @return string
     */
    public function compileEndOnce()
    {}

    /**
     * Compile a boolean value into a raw true / false value for embedding into HTML attributes or JavaScript.
     *
     * @param  bool  $condition
     * @return string
     */
    protected function compileBool($condition)
    {}

    /**
     * Compile a checked block into valid PHP.
     *
     * @param  string  $condition
     * @return string
     */
    protected function compileChecked($condition)
    {}

    /**
     * Compile a disabled block into valid PHP.
     *
     * @param  string  $condition
     * @return string
     */
    protected function compileDisabled($condition)
    {}

    /**
     * Compile a required block into valid PHP.
     *
     * @param  string  $condition
     * @return string
     */
    protected function compileRequired($condition)
    {}

    /**
     * Compile a readonly block into valid PHP.
     *
     * @param  string  $condition
     * @return string
     */
    protected function compileReadonly($condition)
    {}

    /**
     * Compile a selected block into valid PHP.
     *
     * @param  string  $condition
     * @return string
     */
    protected function compileSelected($condition)
    {}

    /**
     * Compile the push statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compilePushIf($expression)
    {}

    /**
     * Compile the else-if push statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileElsePushIf($expression)
    {}

    /**
     * Compile the else push statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileElsePush($expression)
    {}

    /**
     * Compile the end-push statements into valid PHP.
     *
     * @return string
     */
    protected function compileEndPushIf()
    {}
}
