<?php

// Pun: Rector writes `final private const`, which PHP rejects ("Private constant cannot be final"), so a final
// constant stays.
final class A
{
    final protected const FOUR = 4;
}
