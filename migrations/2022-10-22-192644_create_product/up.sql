-- Your SQL goes here
DROP TYPE productStatus;
CREATE TYPE productStatus AS ENUM ('available', 'out', 'pause');

CREATE TABLE IF NOT EXISTS "sellers"  (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  url VARCHAR(255) NOT NULL,
  lastYearSales INT DEFAULT NULL,
  reputation INT DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS "products" (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  url Varchar(255) NOT NULL,
  status productStatus DEFAULT NULL,
  sales Int DEFAULT NULL,
  sellerId UUID references sellers(id)
);
