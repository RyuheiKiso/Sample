package common

import (
	"fmt"
	"os"
	"runtime"
	"strings"
	"time"
)

func logWithLevel(level string, msg string, tags ...string) {
	// タグを[]で連結
	tagStr := ""
	if len(tags) > 0 {
		for _, t := range tags {
			tagStr += "[" + t + "]"
		}
	}
	// 日時・ファイル情報
	now := time.Now().Format("2006/01/02 15:04:05")
	// 呼び出し元ファイル:行番号（logger.goをスキップして本当の呼び出し元を取得）
	// logWithLevel <- logWithLevelAndTags <- Info/Warn/Error <- 呼び出し元
	_, file, line, ok := runtime.Caller(3)
	loc := ""
	if ok {
		shortFile := file
		if idx := strings.LastIndex(file, "/"); idx != -1 {
			shortFile = file[idx+1:]
		}
		loc = fmt.Sprintf("%s:%d", shortFile, line)
	}
	// 出力
	out := os.Stdout
	if level == "ERROR" {
		out = os.Stderr
	}
	fmt.Fprintf(out, "[%s]%s %s %s: %s\n", level, tagStr, now, loc, msg)
}

// Info("メッセージ", タグ...)
func Info(format string, vAndTags ...interface{}) {
	logWithLevelAndTags("INFO", format, vAndTags...)
}

func Warn(format string, vAndTags ...interface{}) {
	logWithLevelAndTags("WARN", format, vAndTags...)
}

func Error(format string, vAndTags ...interface{}) {
	logWithLevelAndTags("ERROR", format, vAndTags...)
}

// format, v..., tags... の形式で受け取る
func logWithLevelAndTags(level string, format string, vAndTags ...interface{}) {
	var v []interface{}
	var tags []string
	n := len(vAndTags)
	// 最後の引数がstringならタグ扱い、それ以外はメッセージ引数
	if n > 0 {
		if tag, ok := vAndTags[n-1].(string); ok {
			tags = append(tags, tag)
			v = vAndTags[:n-1]
		} else {
			v = vAndTags
		}
	}
	msg := fmt.Sprintf(format, v...)
	logWithLevel(level, msg, tags...)
}

// 例: Info("ユーザーID: %d でログイン", userID, "Service")
