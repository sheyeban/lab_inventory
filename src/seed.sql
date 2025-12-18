-- categories
INSERT INTO categories (name) VALUES
('Осциллографы'),
('Мультиметры'),
('Паяльные станции'),
('Блоки питания'),
('Инструменты');

-- vendors
INSERT INTO vendors (name) VALUES
('Rigol'),
('Keysight'),
('UNI-T'),
('Hakko'),
('Mean Well');

-- inventory items
INSERT INTO inventory_items (name, inv_num, status, category_id, vendor_id) VALUES
('Rigol DS1054Z',      'INV-0001', 'available', 1, 1),
('Keysight DSOX1102G', 'INV-0002', 'in_service', 1, 2),
('UNI-T UT61E+',       'INV-0100', 'available', 2, 3),
('Hakko FX-888D',      'INV-0200', 'available', 3, 4),
('Mean Well LRS-350',  'INV-0300', 'written_off', 4, 5);
