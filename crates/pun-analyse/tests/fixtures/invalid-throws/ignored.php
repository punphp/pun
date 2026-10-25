<?php

namespace ThrowsIgnored;

/** @throws bool */
function ignored(): void {} // @phpstan-ignore throws.notThrowable

/** @throws string */
function kept(): void {}
