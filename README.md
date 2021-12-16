# bytecode_interpreter

# Example
               /*
                         * LOAD_VAL 1
         i = 1           * WRITE_VAR ‘i’
                         * READ_VAR ‘i’
         while (i < 5) { * LOAD_VAL 5
             i = i + 1;  * IF_CMP_GE 10
         }               * READ_VAR ‘i’
                         * LOAD_VAL 1
         return i;       * ADD
                         * WRITE_VAR ‘i’
                         * GOTO 2
                         * RETURN_VALUE
                         * */
# Output

    {"Ok":{"variable":"‘i’","value":"5"}}
