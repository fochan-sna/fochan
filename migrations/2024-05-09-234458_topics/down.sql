-- This file should undo anything in `up.sql`
DELETE FROM Topics
WHERE topic_id = 'b1074c65-6006-4858-a8b0-f6ff98b7fe03' OR
      topic_id = '9e1c2867-2ccb-4cef-9066-3b0c96a2d06a' OR
      topic_id = 'f8e3f824-634a-483f-955f-86c998149cab';