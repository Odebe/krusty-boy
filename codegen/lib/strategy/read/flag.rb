module Strategy
  module Read
    class Flag
      def self.call(operand)
        tmp = "cpu.reg.flag_get(#{operand.clean_flag.upcase})"
        tmp = "!#{tmp}" if operand.negative_flag?
        tmp
      end
    end
  end
end
