require "datafusion"

RSpec.describe "SessionContext" do
  describe "#new" do
    it "returns a SessionContext" do
      ctx = Datafusion::SessionContext.new
      expect(ctx.is_a?(Datafusion::SessionContext)).to eq true
    end

    it "runs sql on csv" do
      ctx = Datafusion::SessionContext.new
      ctx.register_csv("csv", "#{File.dirname(__FILE__)}/../fixtures/test.csv")
      results = ctx.sql("SELECT * FROM csv").collect
      expect(results.size).to eq 1
      expect(results[0].to_h).to eq({
        "int" => [1, 2, 3, 4],
        "str" => ["a", "b", "c", "d"],
        "float" => [1.1, 2.2, 3.3, 4.4],
      })
    end
  end
end
