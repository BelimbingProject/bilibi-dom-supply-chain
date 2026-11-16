-- A revision changes on every source-row update, including lifecycle changes.
ALTER TABLE supply_chain_purchasing_orders ADD COLUMN rule_revision bigint NOT NULL DEFAULT 1 CHECK (rule_revision > 0);
CREATE FUNCTION supply_chain_purchasing_rule_revision() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
  NEW.rule_revision := OLD.rule_revision + 1;
  RETURN NEW;
END $$;
CREATE TRIGGER zz_rule_revision BEFORE UPDATE ON supply_chain_purchasing_orders
FOR EACH ROW EXECUTE FUNCTION supply_chain_purchasing_rule_revision();
