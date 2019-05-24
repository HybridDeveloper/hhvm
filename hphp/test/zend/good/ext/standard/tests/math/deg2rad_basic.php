<?php
/*
 * proto float deg2rad(float number)
 * Function is implemented in ext/standard/math.c
 */

require(__DIR__."/allowed_rounding_error.inc");

<<__EntryPoint>> function main() {

$arg_0 = 0.0;
$arg_1 = 90.0;
$arg_2 = 180.0;
$arg_3 = 360.0;


echo "deg2rad $arg_0 = ";
$r0 = deg2rad($arg_0);
var_dump($r0);
if (allowed_rounding_error($r0 ,0 )) {
    echo "Pass\n";
}
else {
    echo "Fail\n";
}

echo "deg2rad $arg_1 = ";
$r1 = deg2rad($arg_1);
var_dump($r1);
if (allowed_rounding_error($r1 ,1.5707963267949 )) {
    echo "Pass\n";
}
else {
    echo "Fail\n";
}
echo "deg2rad $arg_2 = ";
$r2 = deg2rad($arg_2);
var_dump($r2);
if (allowed_rounding_error($r2 ,3.1415926535898 )) {
    echo "Pass\n";
}
else {
    echo "Fail\n";
}
echo "deg2rad $arg_3 = ";
$r3 = deg2rad($arg_3);
var_dump($r3);
if (allowed_rounding_error($r3 ,6.2831853071796 )) {
    echo "Pass\n";
}
else {
    echo "Fail\n";
}
}
