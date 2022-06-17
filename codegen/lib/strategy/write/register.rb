module Strategy
  module Write
    class Register
      def self.call(operand, result)
        clean_name = operand.clean_name.downcase

        tmp = "cpu.reg.#{clean_name}"
        tmp << '()' if clean_name.chars.count > 1
        tmp << " =  #{result}"
      end
    end
  end
end
