<?php

namespace StubScopeApp;

use StubScope\Box;
use StubScope\Child;

/**
 * @param Box<Child> $ok
 * @param Box<\stdClass> $bad
 */
function stub_scope(Box $ok, Box $bad): void
{
}
