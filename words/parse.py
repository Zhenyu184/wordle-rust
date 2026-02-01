import requests
import pdfplumber
import re
import os

def download_pdf(url, save_path):
    print(f"🌐 正在從網頁下載 PDF...")
    response = requests.get(url, stream=True)
    if response.status_code == 200:
        with open(save_path, 'wb') as f:
            f.write(response.content)
        print("✅ 下載成功！")
    else:
        print(f"❌ 下載失敗，狀態碼：{response.status_code}")
        return False
    return True

def extract_wordle_words(pdf_path, output_txt):
    print(f"📂 正在解析檔案：{pdf_path}")
    wordle_words = []
    
    # 正規表達式：匹配單字與詞性 (例如: apple n.)
    word_pattern = re.compile(r'([a-zA-Z\-\s\']+)\s+(n\.|v\.|adj\.|adv\.|prep\.|conj\.|pron\.|art\.|det\.|aux\.)')

    try:
        with pdfplumber.open(pdf_path) as pdf:
            for page in pdf.pages:
                text = page.extract_text()
                if not text: continue
                
                matches = word_pattern.findall(text)
                for match in matches:
                    word = match[0].strip().lower()

                    if word.isalpha():
                        wordle_words.append(word)

        # 去重並排序
        wordle_words = sorted(list(set(wordle_words)))

        with open(output_txt, "w", encoding="utf-8") as f:
            for word in wordle_words:
                f.write(f"{word}\n")
        
        print(f"✅ 處理完成！共提取 {len(wordle_words)} 個 Wordle 單字。")
        print(f"💾 檔案已儲存至：{output_txt}")

    except Exception as e:
        print(f"❌ 解析錯誤: {e}")

if __name__ == "__main__":
    # 大考中心 111學年度起適用詞彙表連結
    TARGET_URL = "https://www.ceec.edu.tw/files/file_pool/1/0k213571061045122620/%E9%AB%98%E4%B8%AD%E8%8B%B1%E6%96%87%E5%8F%83%E8%80%83%E8%A9%9E%E5%BD%99%E8%A1%A8%28111%E5%AD%B8%E5%B9%B4%E5%BA%A6%E8%B5%B7%E9%81%A9%E7%94%A8%29.pdf"
    TEMP_PDF = "temp_words.pdf"
    OUTPUT_FILE = "words.txt"

    if download_pdf(TARGET_URL, TEMP_PDF):
        extract_wordle_words(TEMP_PDF, OUTPUT_FILE)
        
        # (選擇性) 處理完後刪除暫存的 PDF
        # os.remove(TEMP_PDF)